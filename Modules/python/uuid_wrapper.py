# uuid를 문자열로 저장 할 시 메모리가 너무 커져서, 바이트를 감싸주는 래퍼를 생성함
# 10000000개의 uuid를 저장할 시 물리메모리
# str - 1175MB
# byte - 710MB, 단 문자열 출력 불가
# UUIDWrapper - 871MB
class UUIDWrapper(bytes):
    """
    128비트의 데이터(16바이트)를 래핑해주는 래퍼
    """

    def __new__(self, *args, **kwargs):
        """
        기본 생성자

        Returns:
            _type_: 추가적인 처리가 필요하지 않아 args와 kwargs를 그대로 넘겨줌
        """
        return super().__new__(self, *args, **kwargs)

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
