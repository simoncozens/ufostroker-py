import ufoLib2
f = ufoLib2.Font("tests/data/Scurve.ufo")

def test_cws():
	from ufostroker import constant_width_stroke
	a = f["A"]
	constant_width_stroke(a, 100)
	assert len(a) == 1
	assert len(a[0]) == 36

	b = f["B"]
	constant_width_stroke(b, 100)
	assert len(b) == 4
	assert len(b[0]) == 12
