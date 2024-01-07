# Regex Benchmark: Performance Visualization Tool for Regular Expressions


- [Regex Benchmark: Performance Visualization Tool for Regular Expressions](#regex-benchmark-performance-visualization-tool-for-regular-expressions)
  - [Description](#description)
  - [Why Regex Performance Matters?](#why-regex-performance-matters)
  - [Examples](#examples)
  - [Installation](#installation)
    - [Linux](#linux)
    - [Windows](#windows)
  - [Usage](#usage)
  - [Contributing](#contributing)


## Description

Regex Benchmark is a utility tool aimed at demystifying the performance of regular expressions. It provides a visual representation of how different regex patterns perform, especially useful when dealing with complex expressions where performance implications are not immediately apparent.

## Why Regex Performance Matters?

Understanding the performance of regular expressions is crucial for several reasons:

* **Security** - Avoding [ReDoS attack](https://owasp.org/www-community/attacks/Regular_expression_Denial_of_Service_-_ReDoS) attacks.
* **Performance** - Ensuring efficient execution, crucial for user experience and server load management.
* **Cost** - Minimising computational resources, especially in cloud-based solutions.
* **Real-World Lessons** - Learning from incidents like [Cloudflare's global outage](https://blog.cloudflare.com/details-of-the-cloudflare-outage-on-july-2-2019/).


## Examples

Let's suppose I want to test the performance of the following three request expressions:

1. `=`
2. `.*(=).*`
3. `.?(=).*`

All of these expressions are valid, and they all do the same thing. In this example, it's easy to see that the first expression is the most efficient. However, in more complex expressions, it's not always easy to see which expression is the most efficient. This is where `regex-benchmark` comes in.

We'll run test each of the regex expressions using the following options:

* `--max-length 100000` - The max string length to test will be 100,000.
* `--step-size 10` - The step size will be 10, meaning the string length will increase by 10 each iteration.
* `--num-tests 5` - We will run each test 5 times plotting each result onto the graph.
* `--required-str x=xxxxxxxxxxxxxx` - The random strings that are generated will have the string `x=xxxxxxxxxxxxxx` somewhere in them.
* `--method find` - We want to locate the "=" in the string. This isn't at all useful in a real life example, but it's a nice way to demonstrate the performance of the regex expressions.

Let's run the tests:

```bash
regex-benchmark \
  --regex '=' \
  --max-length 100000 \
  --step-size 10 \
  --num-tests 5 \
  --required-str x=xxxxxxxxxxxxxx \
  --method find
```

![Example 1 results](./docs/examples/img/example-test-1.png)

```bash
regex-benchmark \
  --regex '.*(=).*' \
  --max-length 100000 \
  --step-size 10 \
  --num-tests 5 \
  --required-str x=xxxxxxxxxxxxxx \
  --method find
```

![Example 2 results](./docs/examples/img/example-test-2.png)

```bash
regex-benchmark \
  --regex '.?(=).*' \
  --max-length 100000 \
  --step-size 10 \
  --num-tests 5 \
  --required-str x=xxxxxxxxxxxxxx \
  --method find
```

![Example 3 results](./docs/examples/img/example-test-3.png)

Although all three expressions are valid and we can retrieve the "=" from the results, we can see that the performance of each expression is very different.

The first expression is the most efficient. Although we do have a couple of outliers, the performance is very consistent giving us a good indication that the result is constant time.

The second expression is the least efficient. We can see a linear growth with a increase in spread as the input size increases. **This might look like a time complexity of O(n), however this isn't actually true. The time complexity is actually O(n^3) in terms of how many steps actually need to be executed. This graph instead represents the actual time to perform the regex search**.

The final expression is the most interesting. We can see that it is slower than the first expression, but faster than the second. The biggest difference is that the spread is absolutely massive!


## Installation

### Linux

1. Download the latest `regex-benchmark` binary from [releases page](https://github.com/Salaah01/regex-benchmark/releases).
2. Ubuntu users will need to install additional dependencies:
    ```bash
    sudo apt install pkg-config libfreetype6-dev libfontconfig1-dev
    ```
3. Run the binary in the terminal with the `--help` flag to see the available options:
    ```bash
    ./regex-benchmark --help
    ```

### Windows

1. Download the latest `regex-benchmark.exe` binary from [releases page](https://github.com/Salaah01/regex-benchmark/releases).
2. Run the binary in Powershell/CMD with the `--help` flag to see the available options:
    ```powershell
    .\regex-benchmark.exe --help
    ```

## Usage

One you have installed the binary, you can run it with the `--help` flag to see the available options:

```bash
./regex-benchmark --help
```

For a quick start, you can try running one of the regex expressions from the [examples](#examples) above:

```bash
regex-benchmark \
  --regex '.?(=).*' \
  --max-length 100000 \
  --step-size 10 \
  --num-tests 5 \
  --required-str x=xxxxxxxxxxxxxx \
  --method find
```

## Contributing

Contributions are welcome! Whether it's reporting bugs, suggesting features, or submitting code changes, please refer to our [contributing guide](./CONTRIBUTING.md). We appreciate contributions from developers of all skill levels.
