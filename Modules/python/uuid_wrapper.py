# uuid를 문자열로 저장 할 시 메모리가 너무 커져서, 바이트를 감싸주는 래퍼를 생성함
# 10000000개의 uuid를 저장할 시 물리메모리
# str - 1175MB
# byte - 710MB, 단 문자열 출력 불가
# UUIDWrapper - 871MB

class UUIDWrapperInt(int):
    """
    128비트의 데이터(16바이트)를 래핑해주는 래퍼
    UUID 내부 str을 할 때 사용되는 int데이터를 기반으로 래핑한다.
    str(data)를 했을떄의 속도가 많이 개선됨
    """

    def __new__(cls, *args, **kwargs):
        """
        기본 생성자
        
        Note. 래퍼 생성자의 *args, **kwargs를 data로 특정지어서 적용할 경우 시간 소요가 1.7배 정도로 줄어듬
        Note. 래퍼 생성자를 삭제하면 시간 소요가 1.2배정도로 줄어듬
        ```python
        # 원본의 2배 정도 시간 소요
        def __new__(cls, *args, **kwargs):
            return super().__new__(cls, *args, **kwargs)
        # 원본의 1.7배 정도 시간 소요
        def __new__(cls, data: int):
            return super().__new__(cls, data)
        # 원본의 1.2배 정도 시간 소요
        # def __new__(cls, data: int):
        #     return super().__new__(cls, data)
        ```

        Returns:
            _type_: 추가적인 처리가 필요하지 않아 args와 kwargs를 그대로 넘겨줌
        """
        return super().__new__(cls, *args, **kwargs)

    def __str__(self) -> str:
        """
        str(data)를 했을때 나타나는 데이터

        Returns:
            str: UUID형식에 맞는 데이터
        """
        data = '%032x' % self
        return '%s-%s-%s-%s-%s' % (
            data[:8], data[8:12], data[12:16], data[16:20], data[20:])

    def __repr__(self) -> str:
        """
        data 를 실행했을때 출력되는 데이터

        Returns:
            str: UUID 형식에 맞는 데이터
        """
        data = '%032x' % self
        return f"UUID Wrapper ({ data })"


class UUIDWrapperByte(bytes):
    """
    128비트의 데이터(16바이트)를 래핑해주는 래퍼
    """

    def __new__(cls, *args, **kwargs):
        """
        기본 생성자

        Returns:
            _type_: 추가적인 처리가 필요하지 않아 args와 kwargs를 그대로 넘겨줌
        """
        return super().__new__(cls, *args, **kwargs)

    def __str__(self) -> str:
        """
        str(data)를 했을때 나타나는 데이터

        Returns:
            str: UUID형식에 맞는 데이터
        """
        data = super()
        return '%s-%s-%s-%s-%s' % (
            data[:8], data[8:12], data[12:16], data[16:20], data[20:])

    def __repr__(self) -> str:
        """
        data 를 실행했을때 출력되는 데이터

        Returns:
            str: UUID 형식에 맞는 데이터
        """
        iterator = super().__iter__()
        return f"UUID Wrapper ({next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x} {next(iterator):x})"
