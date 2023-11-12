class Foo:
    def __init__(self, bar: list | str):
        self.bar = bar

    def __str__(self):
        if type(self.bar) == list:
            return "\n".join([f">>{i}" for i in self.bar])
        else:
            return f"{self.bar}"


my_foo = Foo("hi")

my_other_foo = Foo([Foo("hey") for _ in range(5)])

my_next_foo = Foo([my_foo, my_other_foo])

print(my_next_foo)
