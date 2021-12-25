

def row36_to_53(w, z, y, z):
	#36 inp w
	w = i2

	#37	mul x 0
	#38	add x z
	#39	mod x 26
        x = z % 26

	#40	div z 1 <- Ignore

	#41	add x 13
	#42	eql x w
	#43	eql x 0
	x = (z % 26) + 13 == i2

	#44	mul y 0
	#45	add y 25
	#46	mul y x
	#47	add y 1
	y = 26

	#48	mul z y
	z = z * 26

	#49	mul y 0
	#50	add y w
	#51	add y 5
	#52	mul y x
	y = x * (i2*5)

	#53	add z y
	z = z + y
