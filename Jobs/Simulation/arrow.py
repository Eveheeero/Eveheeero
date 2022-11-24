class Arrow:
    """
    중력에 영향을 받는 물체의 Z축 방향의 운동 속도를 계산하는 클래스
    """

    def __init__(self) -> None:
        """
        객체 생성
        """
        # 현재 운동 속도
        # 양수면 윗 방향으로 움직이며, 음수면 아래 방향으로 움직인다.
        self._acc = 0
        pass

    def is_up(self) -> bool:
        """
        Z축 운동방향이 윗 방향을 향하는지 확인

        Returns:
            bool: Z축의 운동 방향이 윗 방향이면 True, 아니면 False
        """
        if self._acc > 0:
            return True
        else:
            return False

    def get_acc(self) -> float:
        """
        현재의 운동 속도를 반환

        ## Note
        내부적으로, 운동속도에 가속도를 추가해줍니다.

        Returns:
            float: 현재의 운동 속도, 양수면 윗 방향으로 움직이며, 음수면 아래 방향으로 움직인다.
        """
        self._event()
        return self._acc

    def bounce(self, bounce: float) -> None:
        """
        틩겼을 때의 운동 속도를 계산한다.
        bound 수치가 양수일 경우, 아래에서 틩겨 올라간 것이며,
        bound 수치가 음수일 경우, 위에서 찍힌 것이다.

        Args:
            bounce (float): 틩긴 운동 속도.
        """
        if bounce > 0:
            self._acc = abs(self._acc) + bounce
        else:
            self._acc = -abs(self._acc) + bounce
        pass

    def has_bounced_to_ground(self, now_position: float) -> float:
        """
        바탁에 틩겼을 때의 운동 속도를 계산한다.

        Args:
            now_position (float): get_acc로 인해 연산된 위치

        Returns:
            float: 물체가 바닥에 틩긴 후, 얼마만큼의 거리를 올라갔는지를 반환한다.
        """
        # 작성하기
        return 0

    def _event(self) -> None:
        """
        운동 속도에 가속도를 추가한다.
        """
        # 중력가속도 합산
        self._acc -= 9.8
        pass
