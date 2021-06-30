from .ufostroker import constant_width_stroke as cws_rust


def constant_width_stroke(
    glyph,
    width,
    startcap="round",
    endcap="round",
    jointype="bevel",
    remove_internal=False,
    remove_external=False,
):
    """Applies a constant-width stroke effect to a glyph, in place.

    Parameters:
        glyph: A ufoLib2 or defcon glyph object
        width: The stroke width, in points.
        startcap: Cap to add at the start of the stroke (One of: "round", "square")
        endcap: Cap to add at the end of the stroke (One of: "round", "square")
        jointype: Joining type (One of: "round", "bevel", "mitre")
        remove_internal: Remove the internal path when stroking closed curves
        remove_external: Remove the external path when stroking closed curves

    Returns nothing, but modifies the glyph.
    """

    if not startcap in ["round", "square"]:
        raise ValueError("Unknown start cap type")
    if not endcap in ["round", "square"]:
        raise ValueError("Unknown end cap type")
    if not jointype in ["round", "bevel", "mitre"]:
        raise ValueError("Unknown join type")
    list_of_list_of_points = [list(c) for c in list(glyph)]
    res = cws_rust(
        list_of_list_of_points, width, startcap, endcap, jointype, remove_internal, remove_external
    )
    contour_class = glyph[0].__class__
    point_class = glyph[0][0].__class__
    contours = []
    glyph.clearContours()
    for contour in res:
        points = []
        for pt in contour:
            x,y,typ = pt
            if not typ:
                typ = None
            # Unfortunately defcon and ufoLib2 have different Point constructors...
            try:
                point = point_class(x,y,typ)
            except Exception:
                point = point_class((x,y), typ)
            points.append(point)
        contour = contour_class()
        # And contour constructors...
        try:
            contour.extend(points)
        except Exception:
            for point in points:
                contour.appendPoint(point)
        glyph.appendContour(contour)
