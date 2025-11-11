// Shade Framework - Benchmarking System
// Copyright (c) 2025 Shadow Protocol Contributors
// Licensed under MIT License

//! Performance benchmarking for ZK circuits

use crate::core::circuit::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use colored::*;

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_duration: Duration,
    pub avg_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub throughput: f64, // operations per second
}

impl BenchmarkResult {
    pub fn new(name: String, durations: Vec<Duration>) -> Self {
        let iterations = durations.len();
        let total_duration: Duration = durations.iter().sum();
        let avg_duration = total_duration / iterations as u32;
        let min_duration = *durations.iter().min().unwrap();
        let max_duration = *durations.iter().max().unwrap();
        let throughput = iterations as f64 / total_duration.as_secs_f64();

        Self {
            name,
            iterations,
            total_duration,
            avg_duration,
            min_duration,
            max_duration,
            throughput,
        }
    }

    pub fn print(&self) {
        println!("\n{}", format!("Benchmark: {}", self.name).bold().cyan());
        println!("  Iterations: {}", self.iterations);
        println!("  Total time: {:?}", self.total_duration);
        println!("  Average:    {:?}", self.avg_duration);
        println!("  Min:        {:?}", self.min_duration);
        println!("  Max:        {:?}", self.max_duration);
        println!("  Throughput: {:.2} ops/sec", self.throughput);
    }

    pub fn to_json(&self) -> String {
        serde_json::json!({
            "name": self.name,
            "iterations": self.iterations,
            "total_duration_ms": self.total_duration.as_millis(),
            "avg_duration_ms": self.avg_duration.as_millis(),
            "min_duration_ms": self.min_duration.as_millis(),
            "max_duration_ms": self.max_duration.as_millis(),
            "throughput": self.throughput,
        })
        .to_string()
    }
}

/// Benchmark suite
pub struct BenchmarkSuite {
    pub name: String,
    benchmarks: Vec<Box<dyn Benchmark>>,
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            benchmarks: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_benchmark(&mut self, benchmark: Box<dyn Benchmark>) {
        self.benchmarks.push(benchmark);
    }

    pub fn run(&mut self) -> BenchmarkReport {
        println!("\n{}", format!("Running benchmark suite: {}", self.name).bold().green());
        println!("{}\n", "─".repeat(70));

        self.results.clear();

        for benchmark in &self.benchmarks {
            let result = benchmark.run();
            result.print();
            self.results.push(result);
        }

        self.generate_report()
    }

    fn generate_report(&self) -> BenchmarkReport {
        println!("\n{}", "═".repeat(70));
        println!("{}", "Benchmark Summary".bold());
        println!("{}", "═".repeat(70));

        let total_time: Duration = self.results.iter().map(|r| r.total_duration).sum();
        let total_iterations: usize = self.results.iter().map(|r| r.iterations).sum();

        println!("Total benchmarks: {}", self.results.len());
        println!("Total iterations: {}", total_iterations);
        println!("Total time:       {:?}", total_time);
        println!("{}\n", "═".repeat(70));

        BenchmarkReport {
            suite_name: self.name.clone(),
            results: self.results.clone(),
            total_duration: total_time,
        }
    }
}

/// Benchmark report
#[derive(Debug, Clone)]
pub struct BenchmarkReport {
    pub suite_name: String,
    pub results: Vec<BenchmarkResult>,
    pub total_duration: Duration,
}

impl BenchmarkReport {
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::json!({
            "suite_name": self.suite_name,
            "total_duration_ms": self.total_duration.as_millis(),
            "results": self.results.iter().map(|r| {
                serde_json::json!({
                    "name": r.name,
                    "iterations": r.iterations,
                    "avg_duration_ms": r.avg_duration.as_millis(),
                    "throughput": r.throughput,
                })
            }).collect::<Vec<_>>(),
        });

        std::fs::write(path, serde_json::to_string_pretty(&json)?)?;
        Ok(())
    }

    pub fn compare_with(&self, baseline: &BenchmarkReport) {
        println!("\n{}", "Performance Comparison".bold().yellow());
        println!("{}", "─".repeat(70));

        let baseline_map: HashMap<String, &BenchmarkResult> = baseline
            .results
            .iter()
            .map(|r| (r.name.clone(), r))
            .collect();

        for result in &self.results {
            if let Some(baseline_result) = baseline_map.get(&result.name) {
                let speedup = baseline_result.avg_duration.as_secs_f64()
                    / result.avg_duration.as_secs_f64();

                let comparison = if speedup > 1.0 {
                    format!("{:.2}x faster", speedup).green()
                } else {
                    format!("{:.2}x slower", 1.0 / speedup).red()
                };

                println!(
                    "  {}: {} → {} ({})",
                    result.name,
                    format!("{:?}", baseline_result.avg_duration).dimmed(),
                    format!("{:?}", result.avg_duration),
                    comparison
                );
            }
        }

        println!("{}\n", "─".repeat(70));
    }
}

/// Benchmark trait
pub trait Benchmark {
    fn name(&self) -> &str;
    fn run(&self) -> BenchmarkResult;
}

/// Simple benchmark
pub struct SimpleBenchmark {
    name: String,
    iterations: usize,
    operation: Box<dyn Fn() + Sync>,
}

impl SimpleBenchmark {
    pub fn new(
        name: impl Into<String>,
        iterations: usize,
        operation: impl Fn() + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            iterations,
            operation: Box::new(operation),
        }
    }
}

impl Benchmark for SimpleBenchmark {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> BenchmarkResult {
        let mut durations = Vec::with_capacity(self.iterations);

        // Warmup
        for _ in 0..10 {
            (self.operation)();
        }

        // Actual benchmark
        for _ in 0..self.iterations {
            let start = Instant::now();
            (self.operation)();
            durations.push(start.elapsed());
        }

        BenchmarkResult::new(self.name.clone(), durations)
    }
}

/// Circuit benchmark
pub struct CircuitBenchmark {
    name: String,
    iterations: usize,
    circuit_builder: Box<dyn Fn() -> ConstraintSystem + Sync>,
}

impl CircuitBenchmark {
    pub fn new(
        name: impl Into<String>,
        iterations: usize,
        circuit_builder: impl Fn() -> ConstraintSystem + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            iterations,
            circuit_builder: Box::new(circuit_builder),
        }
    }
}

impl Benchmark for CircuitBenchmark {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> BenchmarkResult {
        let mut durations = Vec::with_capacity(self.iterations);

        // Warmup
        for _ in 0..5 {
            (self.circuit_builder)();
        }

        // Actual benchmark
        for _ in 0..self.iterations {
            let start = Instant::now();
            let cs = (self.circuit_builder)();
            durations.push(start.elapsed());

            // Verify it's valid
            assert!(cs.is_satisfied(), "Circuit constraints not satisfied");
        }

        BenchmarkResult::new(self.name.clone(), durations)
    }
}

/// Throughput benchmark (measures operations per second)
pub struct ThroughputBenchmark {
    name: String,
    duration: Duration,
    operation: Box<dyn Fn() + Sync>,
}

impl ThroughputBenchmark {
    pub fn new(
        name: impl Into<String>,
        duration: Duration,
        operation: impl Fn() + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            duration,
            operation: Box::new(operation),
        }
    }
}

impl Benchmark for ThroughputBenchmark {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> BenchmarkResult {
        let start = Instant::now();
        let mut iterations = 0;
        let mut durations = Vec::new();

        while start.elapsed() < self.duration {
            let op_start = Instant::now();
            (self.operation)();
            durations.push(op_start.elapsed());
            iterations += 1;
        }

        BenchmarkResult::new(self.name.clone(), durations)
    }
}

/// Benchmark builder
pub struct BenchmarkBuilder {
    suite: BenchmarkSuite,
}

impl BenchmarkBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            suite: BenchmarkSuite::new(name),
        }
    }

    pub fn bench(
        mut self,
        name: impl Into<String>,
        iterations: usize,
        operation: impl Fn() + Sync + 'static,
    ) -> Self {
        self.suite
            .add_benchmark(Box::new(SimpleBenchmark::new(name, iterations, operation)));
        self
    }

    pub fn bench_circuit(
        mut self,
        name: impl Into<String>,
        iterations: usize,
        circuit: impl Fn() -> ConstraintSystem + Sync + 'static,
    ) -> Self {
        self.suite
            .add_benchmark(Box::new(CircuitBenchmark::new(name, iterations, circuit)));
        self
    }

    pub fn bench_throughput(
        mut self,
        name: impl Into<String>,
        duration: Duration,
        operation: impl Fn() + Sync + 'static,
    ) -> Self {
        self.suite
            .add_benchmark(Box::new(ThroughputBenchmark::new(name, duration, operation)));
        self
    }

    pub fn build(self) -> BenchmarkSuite {
        self.suite
    }

    pub fn run(mut self) -> BenchmarkReport {
        self.suite.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::circuit::primitives::*;

    #[test]
    fn test_simple_benchmark() {
        let bench = SimpleBenchmark::new("field_multiplication", 100, || {
            let a = Field::from(123);
            let b = Field::from(456);
            let _ = a * b;
        });

        let result = bench.run();
        assert_eq!(result.iterations, 100);
        assert!(result.avg_duration.as_nanos() > 0);
    }

    #[test]
    fn test_circuit_benchmark() {
        let bench = CircuitBenchmark::new("simple_mul_circuit", 50, || {
            let mut cs = ConstraintSystem::new();
            let a = cs.alloc_variable(Some(Field::from(3)));
            let b = cs.alloc_variable(Some(Field::from(4)));
            let c = cs.alloc_variable(Some(Field::from(12)));
            cs.enforce_mul(a, b, c);
            cs
        });

        let result = bench.run();
        assert_eq!(result.iterations, 50);
    }

    #[test]
    fn test_benchmark_builder() {
        let report = BenchmarkBuilder::new("Test Suite")
            .bench("field_add", 100, || {
                let a = Field::from(1);
                let b = Field::from(2);
                let _ = a + b;
            })
            .bench("field_mul", 100, || {
                let a = Field::from(3);
                let b = Field::from(4);
                let _ = a * b;
            })
            .run();

        assert_eq!(report.results.len(), 2);
    }

    #[test]
    fn test_throughput_benchmark() {
        let bench = ThroughputBenchmark::new(
            "hash_throughput",
            Duration::from_millis(100),
            || {
                let x = Field::from(42);
                let _ = x * x;
            },
        );

        let result = bench.run();
        assert!(result.iterations > 0);
        assert!(result.throughput > 0.0);
    }
}
