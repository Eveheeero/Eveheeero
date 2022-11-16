class Arrow:
    def __init__(self) -> None:
        self.tick_count = 0
        self.acc = 0
        pass

    def is_up(self) -> bool:
        if self.acc > 0:
            return True
        else:
            return False

    def get_acc(self) -> float:
        self.event()
        return self.acc

    def add_bounce(self, bounce: float) -> None:
        self.acc += bounce
        pass

    def it_bounced(self) -> None:
      # 작성하기
        pass

    def event(self) -> None:
        self.acc -= 9.8
        pass
