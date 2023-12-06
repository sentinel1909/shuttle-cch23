# Shuttle 2023 Christmas Code Hunt

This repo is the home for my completed solutions for the 2023 Christmas Code Hunt, hosted by [Shuttle](https://shuttle.rs).

*It will be public until the morning of December 1st, 2023.*

## 2023-11-27
- completed Challenge -1 tasks 1 and 2

## 2023-11-28
- implemented internal test module for the Challenge -1 tasks 1 and 2
- unable to figure out how to spin up an instance of my Tower router service, without first starting the main binary with `cargo shuttle run`

## 2023-11-29
- re-factored test module using `tower-test`; which is a crate specifically for testing the logic of services implemented using the `tower` crate

## 2023-11-30
- re-factored into a workspace format, with individual crates for each challenge
- set up workspace configuration for the Challenge; binary will continue to live in the minus1 crate, individual challenge endpoints will be in a separate crate for each day of the challenge

# 2023-12-01
- challenge begins, Day 1 tasks released

# 2023-12-04
- solved Day 1 tasks; tests pass locally
- Day 4 tasks released
- mocked up end point for Task 1

# 2023-12-05
- completed Day 4, Part 1
- needed async within the helper function which supports the processing that happens at the `/4/strength` endpoint; incoming body is converted to bytes using the `.into_bytes` method, which is async
    - can't use async in the Tower Service trait
    - a kind person on the Tokio Tower Discord helped get over the hurdle with a helpful re-factor
- re-factored whole projected into the following crates:
    - `common`
    - `minus1`
    - `day 1`
    - `day 4`
- mocked up endpoint for Day 4, Task 2

