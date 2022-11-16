from arrow import Arrow


class Object:
    def __init__(self) -> None:
        self.position = [0, 0, 100]
        self.vector = [0, 0, Arrow()]
        pass

    def set_start_point(self, x: float, y: float, z: float) -> None:
        self.position = [x, y, z]
        pass

    def set_start_point(self, x: float, y: float) -> None:
        self.position = [x, y, 0]
        pass

    def set_move_to(self, x: float, y: float) -> None:
        self.vector[0] = x
        self.vector[1] = y
        pass

    def event(self) -> None:
        self.position[0] += self.vector[0]
        self.position[1] += self.vector[1]
        self.position[2] += self.vector[2].get_acc()
        if self.position[2] < 0:
            arrow = self.vector[2]
            arrow.it_bounced()
        pass
