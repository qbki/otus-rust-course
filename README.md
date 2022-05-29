![](https://github.com/graydrago/otus-rust-course/actions/workflows/checks.yml/badge.svg)

# Subprojects

## [fizzbuzz](./fizzbuzz)

A warm up. The main reason for this subproject is to get a feeling of what rust language and its ecosystem is.

```sh
$ cargo run --bin fizzbuzz
```

Output:

```
1
2
Fizz
4
Buzz
Fizz
...
```


## [smart-home-1](./smart-home-1)

A basic implementation of smart devices that will be used in later parts of the course.

```sh
$ cargo run --bin smart-home-1
```

Output:

```
Outlet: Kitchen
    power: On
    consumptin: 4000.0kW
Thermometer: Outside
    temperature: 23.0°C
```


## [smart-home-2](./smart-home-2/)

A basic implementation of Smart Home infrastructure.

Output:

```
*** Report ***
    Smart home: Home, sweet home
        Room: Deep scary basement
            Outlet: Unknown outlet
                power: Off
                consumption: 0.0kW
            Thermometer: Unknown thermometer
                temperature: 5.0°C
        Room: Kitchen
            Outlet: Fridge
                power: On
                consumption: 4000.0kW
        Room: Living room
            Thermometer: Inside
                temperature: 23.0°C
            Thermometer: Outside
                temperature: -5.0°C

*** List of Rooms ***
Living room
Deep scary basement
Kitchen

*** List of devices from "Deep scary basement"***
Unknown outlet
Unknown thermometer

*** Please copy and paste it into a weakly report ***
Thermometer name: Unknown thermometer
temperature: 4.0°C

Device was not found
```
