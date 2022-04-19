def trigamma(x: float) -> float:
    if math.isnan(x):
        return math.nan
    if x == 0:
        return -math.inf
    if x < 0:
        r = math.pi / math.sin(-math.pi * x)
        return -trigamma(1 - x) + (r * r)
    if x <= 0.0001:
        return (1 / (x * x)) + 1.6449340668482264 + (1.6449340668482264 * x)
    result = 0
    while x < 8:
        result += 1 / (x * x)
        x += 1
    if x >= 12:
        r = 1 / (x * x)
        result += (0.5 * r) + ((1.0 + (r * (0.16666666666666666 + (r * (-0.03333333333333333 + (r *
                                                                  (0.023809523809523808 + (
                                                                        r * (
                                                                            -0.03333333333333333 + (
                                                                                r * 0.07575757575757576)))))))))) / x)
    return result
