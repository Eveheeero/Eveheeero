from arrow import Arrow


class Object:
    """
    중력을 받는 물체 템플릿 클래스
    """

    def __init__(self) -> None:
        """
        물체 생성
        """
        # 물체의 위치 [X, Y, 높이]
        self._position = [0, 0, 100]
        # 물체의 운동 방향 [X, Y, Z]
        self._vector = [0, 0, Arrow()]
        pass

    def set_start_point(self, x: float, y: float, z: float = 0.0) -> None:
        """
        객체 시작지점 설정

        Args:
            x (float): 시작지점의 X좌표
            y (float): 시작지점의 Y좌표
            z (float, optional): 시작지점의 높이. Defaults to 0.0.
        """
        self._position = [x, y, z]
        pass

    def set_move_to(self, x: float, y: float) -> None:
        """
        객체의 이동방향 설정

        Args:
            x (float): 이동방향의 X좌표
            y (float): 이동방향의 Y좌표
        """
        self._vector[0] = x
        self._vector[1] = y
        pass

    def event(self) -> None:
        """
        각 틱마다의 계산

        해당 부분에서는 물체의 위치 및 중력가속도를 계산한다.
        """
        self._position[0] += self._vector[0]
        self._position[1] += self._vector[1]
        self._position[2] += self._vector[2].get_acc()
        if self._position[2] < 0:
            arrow = self._vector[2]
            self._position[2] = arrow.has_bounced_to_ground(self._position[2])
        pass

    def get_position(self) -> tuple[float, float, float]:
        return tuple(self._position)
