def determinant(a, b, c, d):
    return a * d - b * c


def part1(input):
    hailstones = input.splitlines()
    hailstones = [
        list(map(int, h.replace("@", " ").replace(",", " ").split()))
        for h in hailstones
    ]

    area_min = 200000000000000
    area_max = 400000000000000

    answer = 0
    for i, h1 in enumerate(hailstones):
        for h2 in hailstones[:i]:
            x1, y1 = h1[0], h1[1]
            x2, y2 = h2[0], h2[1]
            u1, v1 = h1[3], h1[4]
            u2, v2 = h2[3], h2[4]

            # x1 + u1 * t = x2 + u2 * s
            # y1 + v1 * t = y2 + v2 * s
            # u1 * t - u2 * s = x2 - x1
            # v1 * t - v2 * s = y2 - y1

            # Cramer's rule
            det = determinant(u1, -u2, v1, -v2)
            if det == 0:
                continue

            t = determinant(x2 - x1, -u2, y2 - y1, -v2) / det
            s = determinant(u1, x2 - x1, v1, y2 - y1) / det

            if t < 0 or s < 0:
                continue

            ix = x1 + u1 * t
            iy = y1 + v1 * t

            if area_min <= ix <= area_max and area_min <= iy <= area_max:
                answer += 1

    return answer


def part2(input):
    hailstones = input.splitlines()[:3]
    hailstones = [
        list(map(int, h.replace("@", " ").replace(",", " ").split()))
        for h in hailstones
    ]

    # System 1
    # x + u * t1 = x1 + u1 * t1
    # y + v * t1 = y1 + v1 * t1
    # z + w * t1 = z1 + w1 * t1

    # System 2
    # x + u * t2 = x2 + u2 * t2
    # y + v * t2 = y2 + v2 * t2
    # z + w * t2 = z2 + w2 * t2

    # System 3
    # x + u * t3 = x3 + u3 * t3
    # y + v * t3 = y3 + v3 * t3
    # z + w * t3 = z3 + w3 * t3

    # Unknowns: x, y, z, u, v, w, t1, t2, t3
    # Equations: 9
    # Not linear

    from z3 import Ints, Solver

    x1, y1, z1, u1, v1, w1 = hailstones[0]
    x2, y2, z2, u2, v2, w2 = hailstones[1]
    x3, y3, z3, u3, v3, w3 = hailstones[2]

    s = Solver()
    x, y, z, u, v, w, t1, t2, t3 = Ints("x y z u v w t1 t2 t3")
    s.add(x + u * t1 == x1 + u1 * t1)
    s.add(y + v * t1 == y1 + v1 * t1)
    s.add(z + w * t1 == z1 + w1 * t1)
    s.add(x + u * t2 == x2 + u2 * t2)
    s.add(y + v * t2 == y2 + v2 * t2)
    s.add(z + w * t2 == z2 + w2 * t2)
    s.add(x + u * t3 == x3 + u3 * t3)
    s.add(y + v * t3 == y3 + v3 * t3)
    s.add(z + w * t3 == z3 + w3 * t3)

    s.check()
    m = s.model()

    return m[x].as_long() + m[y].as_long() + m[z].as_long()
