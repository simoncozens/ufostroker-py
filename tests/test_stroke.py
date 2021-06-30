import ufoLib2
f = ufoLib2.Font("tests/data/Scurve.ufo")
a = f["C"]

def test_cws():
	from ufostroker import constant_width_stroke
	constant_width_stroke(a, 100)
	# assert len(a) == 1
	# assert len(a[0]) == 44
	f.save("CWS.ufo")
