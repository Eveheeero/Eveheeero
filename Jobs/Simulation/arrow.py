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
        # 이전 값 연산
        last_position = now_position - self._acc
        last_acc = self._acc + 9.8

        # 언제 바닥에 부딪혔는지 계산
        # 위치가 1에서 -7로 바뀌었다고 하면, 전체 거리는 8이며, 0부터 1까지의 거리는 1이므로 1/8 타이밍으로 바닥에 부딪혔다고 볼 수 있다.
        # 하지만 기존 연산에는 0 이하로 떨어졌을 때에도 계속해서 가속도가 붙었을것이므로, 이에 대한 가중치를 구하기 위해 1.4를 곱한다.
        # 다음 연산은, last_position과  1 - now_position이 양수가 될 것을 기대하여 abs를 제거하였으며, 기존은 abs(last_position) + abs(1 - now_position)이었습니다.
        bounced_timing = pow(
            last_position / (last_position - now_position), 1.4)

        # 바닥에 닿았을 때의 물체의 운동속도
        acc_at_bounce = last_acc - (9.8 * bounced_timing)

        # 바닥에 닿은 뒤, 올라갔을때의 운동속도
        result_acc = -acc_at_bounce - (9.8 * (1 - bounced_timing))

        self._acc = result_acc

        return result_acc

    def _event(self) -> None:
        """
        운동 속도에 가속도를 추가한다.
        """
        # 중력가속도 합산
        self._acc -= 9.8
        pass
