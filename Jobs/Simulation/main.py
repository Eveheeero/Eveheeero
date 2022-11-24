from ball import Ball

if __name__ == "__main__":
    objects = []
    objects.append(Ball())
    while True:
        for one in objects:
            one.event()
        for one in objects:
            one.print()
