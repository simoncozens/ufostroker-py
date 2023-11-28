import py

def datadir():
    return py.path.local(py.path.local(__file__).dirname).join("data")

def test_cws_ufoLib2():
	import ufoLib2
	f = ufoLib2.Font.open(datadir() + "/Scurve.ufo")
	from ufostroker import constant_width_stroke
	a = f["A"]
	constant_width_stroke(a, width=10, height=10, startcap="square", endcap="square")
	b = f["B"]
	constant_width_stroke(b, width=10, height=10, startcap="square", endcap="square")
	c = f["C"]
	constant_width_stroke(c, width=10, height=10, startcap="square", endcap="square")
	#f.save("saved2.ufo", overwrite=True)
	assert len(a) == 1
	assert len(a[0]) == 48

	assert len(b) == 4
	assert len(b[0]) == 12


def test_cws_defcon():
	import defcon
	f = defcon.Font(datadir() + "/Scurve.ufo")
	from ufostroker import constant_width_stroke
	a = f["A"]

	constant_width_stroke(a, 100)
	assert len(a) == 1
	assert len(a[0]) == 54

	b = f["B"]
	constant_width_stroke(b, 100)
	assert len(b) == 4
	assert len(b[0]) == 12


def test_cws_remove_internal():
	import ufoLib2
	f = ufoLib2.Font.open(datadir() + "/Scurve.ufo")
	from ufostroker import constant_width_stroke
	b = f["B"]
	constant_width_stroke(b, 100, remove_internal = True)
	assert len(b) == 2
	assert len(b[0]) == 12
