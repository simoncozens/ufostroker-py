use glifparser::glif::VWSHandle;
use glifparser::Handle;
use glifparser::Outline;
use glifparser::Point;
use glifparser::PointData;
use glifparser::VWSContour;
use glifparser::{
    glif::{InterpolationType, MFEKPointData},
    CapType, JoinType, PointType,
};
use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pyfunction;
use MFEKmath::variable_width_stroke;
use MFEKmath::variable_width_stroking::VWSSettings;
use MFEKmath::Piecewise;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn ufostroker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(constant_width_stroke))?;
    Ok(())
}

struct CWSSettings {
    vws_settings: VWSSettings,
    width: f64,
    jointype: JoinType,
    startcap: CapType,
    endcap: CapType,
    remove_internal: bool,
    remove_external: bool,
}

fn constant_width_stroke_internal(
    path: Outline<MFEKPointData>,
    settings: &CWSSettings,
) -> Outline<MFEKPointData> {
    let vws_contour = VWSContour {
        // id: 0,
        join_type: settings.jointype,
        cap_start_type: settings.startcap,
        cap_end_type: settings.endcap,
        handles: vec![], // to be populated based on number of points
        remove_internal: settings.remove_internal,
        remove_external: settings.remove_external,
    };

    // convert our path and pattern to piecewise collections of beziers
    let piece_path = Piecewise::from(&path);
    let mut output_outline = Vec::new();

    let mut vws_contours = vec![vws_contour; path.len()];

    let vws_handle = VWSHandle {
        left_offset: settings.width / 2.0,
        right_offset: settings.width / 2.0,
        tangent_offset: 0.0,
        interpolation: InterpolationType::Linear,
    };

    for (cidx, contour) in path.iter().enumerate() {
        let pointiter = contour.iter().enumerate();

        for (_, _) in pointiter {
            vws_contours[cidx].handles.push(vws_handle);
        }
        vws_contours[cidx].handles.push(vws_handle);
    }

    let iter = piece_path.segs.iter().enumerate();
    for (i, pwpath_contour) in iter {
        let vws_contour = &vws_contours[i];

        let results = variable_width_stroke(&pwpath_contour, &vws_contour, &settings.vws_settings);
        for result_contour in results.segs {
            output_outline.push(result_contour.to_contour());
        }
    }
    output_outline
}

fn get_point_type(point: &PyAny) -> PyResult<&str> {
    point
        .getattr("type")
        .unwrap_or_else(|_| point.getattr("segmentType").unwrap())
        .extract()
}

fn py_ufo_glyph_to_outline(contours: &PyList) -> Outline<MFEKPointData> {
    let mut out: Outline<MFEKPointData> = vec![];
    for contour in contours.iter() {
        let points: &PyList = contour.downcast::<PyList>().unwrap();
        let mut out_contour = vec![];
        for i in 0..(points.len() as isize) {
            let next = (i + 1) % (points.len() as isize);
            let prev = if i - 1 < 0 {
                (points.len() as isize) - 1
            } else {
                i - 1
            };
            let point = points.get_item(i);
            let typ: PyResult<&str> = get_point_type(point);
            if typ.is_err() {
                continue;
            }

            let next_node = points.get_item(next);
            let prev_node = points.get_item(prev);

            let x: f32 = point.getattr("x").unwrap().extract().unwrap();
            let y: f32 = point.getattr("y").unwrap().extract().unwrap();
            let ptype = match typ {
                Ok("move") => PointType::Move,
                Err(_) => PointType::OffCurve,
                Ok("curve") => PointType::Curve,
                Ok("line") => PointType::Line,
                _ => PointType::Undefined,
            };
            let mut mfek_point = Point::from_x_y_type((x, y), ptype);
            let next_typ: PyResult<&str> = get_point_type(next_node);
            if next_typ.is_err() {
                mfek_point.a = Handle::At(
                    next_node.getattr("x").unwrap().extract().unwrap(),
                    next_node.getattr("y").unwrap().extract().unwrap(),
                )
            }
            let prev_typ: PyResult<&str> = get_point_type(prev_node);
            if prev_typ.is_err() {
                mfek_point.b = Handle::At(
                    prev_node.getattr("x").unwrap().extract().unwrap(),
                    prev_node.getattr("y").unwrap().extract().unwrap(),
                )
            }
            out_contour.push(mfek_point);
        }
        out.push(out_contour);
    }
    out
}

fn outline_to_pyish_contours(outline: Outline<MFEKPointData>) -> Vec<Vec<(f32, f32, String)>> {
    let mut out_contours = vec![];
    for contour in outline.iter() {
        let mut out_contour = vec![];
        for point in contour.iter() {
            if let glifparser::Handle::At(x, y) = point.b {
                out_contour.push((x, y, "".to_string()));
            }
            out_contour.push((
                point.x,
                point.y,
                match point.ptype {
                    PointType::OffCurve => "",
                    PointType::Curve => "curve",
                    PointType::Line => "line",
                    PointType::Move => "move",
                    _ => "",
                }
                .to_string(),
            ));
            if let glifparser::Handle::At(x, y) = point.a {
                out_contour.push((x, y, "".to_string()));
            }
        }
        out_contours.push(out_contour);
    }
    out_contours
}

fn str_to_jointype(s: &str) -> JoinType {
    match s {
        "bevel" => JoinType::Bevel,
        "miter" => JoinType::Miter,
        "round" => JoinType::Round,
        _ => unimplemented!(),
    }
}

fn str_to_cap(s: &str) -> CapType {
    match s {
        "round" => CapType::Round,
        "square" => CapType::Square,
        _ => CapType::Custom,
    }
}

#[pyfunction]
fn constant_width_stroke(
    contours: &PyList,
    width: f64,
    startcap: &str,
    endcap: &str,
    jointype: &str,
    remove_internal: bool,
    remove_external: bool,
) -> Vec<Vec<(f32, f32, String)>> {
    let vws_settings = VWSSettings {
        cap_custom_end: None,
        cap_custom_start: None,
    };
    let settings = CWSSettings {
        vws_settings,
        width,
        startcap: str_to_cap(startcap),
        endcap: str_to_cap(endcap),
        jointype: str_to_jointype(jointype),
        remove_internal,
        remove_external,
    };
    outline_to_pyish_contours(constant_width_stroke_internal(
        py_ufo_glyph_to_outline(contours),
        &settings,
    ))
}
