import object


class Ball(object.Object):
    def __init__(self) -> None:
        super().__init__()
        pass

    def event(self) -> None:
        super().event()
        pass

    def print(self) -> None:
        print(self.get_position())
