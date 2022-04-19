import math
# run메소드가 있는 MasterFunction과 ObjectFunction, DerivateFunction 클래스가 필요합니다.


class RaphsonClass:
    accuracy = 10e-7    # 설정이 필요합니다.
    iterationsPerTry = 1000
    iterationsTotal = 17000
    timePerTry = 100
    timeTotal = 1000
    preparedStartValues = None
    MasterFunction = None  # 뉴턴 랩슨 메인 함수입니다. run(float, float)형태의 메소드가 존재합니다.
    ObjectFunction = None
    DerivateFunction = None

    def __init__(self, accuracy=10e-7, iterations_per_try=1000, iterations_total=17000, time_per_try=100,
                 time_total=1000):
        """
        뉴턴랩슨 설정을 변경합니다.
        :param accuracy: 뉴턴랩슨법 정확도
        :param iterations_per_try: 주기당 반복 횟수 헌계
        :param iterations_total: 반복 총 합 헌계
        :param time_per_try: 주기당 시간 헌계
        :param time_total: 전체 시간 헌계
        :return:
        """
        self.accuracy = accuracy
        self.iterationsPerTry = iterations_per_try
        self.iterationsTotal = iterations_total
        self.timePerTry = time_per_try
        self.timeTotal = time_total

    def set_master_function(self, function=None) -> None:
        self.MasterFunction = function
        return

    def set_object_function(self, function=None) -> None:
        self.ObjectFunction = function
        return

    def set_derivate_function(self, function=None) -> None:
        self.DerivateFunction = function
        return

    def run(self, start: list[float, float]) -> list[float, float]:  # 뉴턴 랩슨 시작 좌표
        import warnings
        warnings.filterwarnings("ignore")   # 뉴턴랩슨 함수 내부에서는 오류가 많이 발생합니다. 이에 대한 예외처리가 되어있으며, 오류메세지는 불필요합니다.
        import time
        solution: list[float, float] = None  # 결과값
        objects: list[float, float]  # 이동값
        derivatives: list[float, float, float, float]  # 편미분 x1, x2, y1, y2값
        total_start = time.time()  # 시작시간
        total_iterations: int = 0  # 반복횟수 제약사항
        prepared_start_value_offset: int = 0  # 위의 preparedStartValues를 불러오기 위한 오프셋변수
        outer_break_flag = False  # 외부 반복문을 탈출하기 위한 플래그

        iterationsTotal = self.iterationsTotal  # 자주 호출이 필요한 전역변수를 지역변수로 전환한다.
        preparedStartValues = self.preparedStartValues
        MasterFunction = self.MasterFunction
        ObjectFunction = self.ObjectFunction
        accuracy = self.accuracy
        DerivateFunction = self.DerivateFunction
        iterationsPerTry = self.iterationsPerTry
        timeTotal = self.timeTotal
        timePerTry = self.timePerTry

        while total_iterations <= iterationsTotal:  # outer루프
            start_per_try = time.time()  # 주기 시작 시간 초기화
            iterations = 0  # 주기 반복 횟수 초기화

            if solution is None:  # 실행을 한번도 안 했으면 초기값으로 설정한다.
                solution = start
            else:
                if preparedStartValues is not None:
                    solution = [preparedStartValues[prepared_start_value_offset][0],
                                preparedStartValues[prepared_start_value_offset][1]]
                    prepared_start_value_offset += 1
                else:
                    import random
                    solution = [(random.random() * 2 - 1) * start[0], (random.random() * 2 - 1) * start[1]]
            while True:  # inner루프
                total_iterations += 1
                if MasterFunction is None:  # MasterFunction이 지정되어 있지 않으면 ObjectFunction과 DerivateFunction으로 연산한다.
                    objects = ObjectFunction.run(solution[0], solution[1])
                    if abs(objects[0]) <= accuracy and abs(objects[1]) <= accuracy: # 이동거리가 설정한 정확도보다 자세하면 멈춘다.
                        return solution
                    derivatives = DerivateFunction.run(solution[0], solution[1])    # 아닐시 편미분을 계산한다.
                else:
                    results = MasterFunction.run(solution[0], solution[1])  # MasterFunction은 이동거리와 편미분을 동시에 계산한다,
                    objects = results[0]
                    derivatives = results[1]
                    if abs(objects[0]) <= accuracy and abs(objects[1]) <= accuracy:
                        return solution
                # 이전 함수와의 차이점 계산.
                # 뉴턴랩슨의 derivatives.inverse의 내용
                scalar = 1 / (derivatives[0] * derivatives[3] - derivatives[1] * derivatives[2])
                derivatives = [derivatives[3] * scalar, -derivatives[1] * scalar, -derivatives[2] * scalar,
                               derivatives[0] * scalar]
                # object.times(derivatives)의 내용
                objects = [objects[0] * derivatives[0] + objects[1] * derivatives[1],
                           objects[0] * derivatives[2] + objects[1] * derivatives[3]]
                # solution.minus(object)의 내용
                solution[0] -= objects[0]
                solution[1] -= objects[1]

                now_time = time.time() # 반복 완료 후, 제약사항 검사
                if now_time - total_start > timeTotal or (
                        preparedStartValues is not None and prepared_start_value_offset == len(preparedStartValues)):
                    outer_break_flag = True
                    break
                    # 전체 시간이 초과하였거나, 초기값이 다 떨어졌으면 outer루프를 종료한다.
                if solution[0] < 0:
                    break
                    # x값이 0보다 작으면 반복을 중지하는 제약사항이다.
                    # 추가적으로 다른 제약사항이 들어갈 수 있다.
                iterations += 1
                if math.isnan(solution[0]) or math.isnan(
                        solution[1]) or iterations > iterationsPerTry or now_time - start_per_try > timePerTry:
                    break
                    # 결과값이 오류가나거나 반복당 시간이 지나있을 시, 다음 주기를 돈다.
            if outer_break_flag:
                break
        return [math.nan, math.nan]
