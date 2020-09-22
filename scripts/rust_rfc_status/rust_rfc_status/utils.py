import functools

def retryable(func):
    functools.wraps(func)

    def __inner(*args, **kwargs):
        while True:
            try:
                return func(*args, **kwargs)
            except IOError as e:
                pass

    return __inner