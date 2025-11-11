// Shroud Framework - Testing Framework
// Copyright (c) 2025 Shroud Protocol Contributors
// Licensed under MIT License

//! Comprehensive testing framework for ZK circuits

use crate::core::circuit::*;
use std::time::{Duration, Instant};
use colored::*;

/// Test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration: Duration,
    pub error: Option<String>,
}

impl TestResult {
    pub fn success(name: String, duration: Duration) -> Self {
        Self {
            name,
            passed: true,
            duration,
            error: None,
        }
    }

    pub fn failure(name: String, duration: Duration, error: String) -> Self {
        Self {
            name,
            passed: false,
            duration,
            error: Some(error),
        }
    }

    pub fn print(&self) {
        let status = if self.passed {
            "PASS".green()
        } else {
            "FAIL".red()
        };

        let time = format!("({:?})", self.duration).dimmed();

        println!("  {} {} {}", status, self.name, time);

        if let Some(ref error) = self.error {
            println!("    {}: {}", "Error".red(), error);
        }
    }
}

/// Test suite for circuits
pub struct TestSuite {
    name: String,
    tests: Vec<Box<dyn CircuitTest>>,
    results: Vec<TestResult>,
}

impl TestSuite {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tests: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Add a test to the suite
    pub fn add_test(&mut self, test: Box<dyn CircuitTest>) {
        self.tests.push(test);
    }

    /// Run all tests
    pub fn run(&mut self) -> TestReport {
        println!("\n{} {}\n", "Running test suite:".bold(), self.name.bold());

        self.results.clear();

        for test in &self.tests {
            let start = Instant::now();
            let result = match test.run() {
                Ok(_) => TestResult::success(test.name().to_string(), start.elapsed()),
                Err(e) => TestResult::failure(test.name().to_string(), start.elapsed(), e.to_string()),
            };

            result.print();
            self.results.push(result);
        }

        self.generate_report()
    }

    /// Generate test report
    fn generate_report(&self) -> TestReport {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        let total_duration = self.results.iter().map(|r| r.duration).sum();

        println!("\n{}", "─".repeat(60));
        println!("{}", "Test Summary".bold());
        println!("{}", "─".repeat(60));
        println!("Total:  {}", total);
        println!("Passed: {}", passed.to_string().green());
        println!("Failed: {}", failed.to_string().red());
        println!("Time:   {:?}", total_duration);
        println!("{}\n", "─".repeat(60));

        TestReport {
            suite_name: self.name.clone(),
            total,
            passed,
            failed,
            duration: total_duration,
            results: self.results.clone(),
        }
    }
}

/// Test report
#[derive(Debug, Clone)]
pub struct TestReport {
    pub suite_name: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub duration: Duration,
    pub results: Vec<TestResult>,
}

impl TestReport {
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }

    pub fn is_success(&self) -> bool {
        self.failed == 0
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

impl serde::Serialize for TestReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("TestReport", 6)?;
        state.serialize_field("suite_name", &self.suite_name)?;
        state.serialize_field("total", &self.total)?;
        state.serialize_field("passed", &self.passed)?;
        state.serialize_field("failed", &self.failed)?;
        state.serialize_field("duration_ms", &self.duration.as_millis())?;
        state.serialize_field("success_rate", &self.success_rate())?;
        state.end()
    }
}

/// Circuit test trait
pub trait CircuitTest {
    fn name(&self) -> &str;
    fn run(&self) -> Result<(), CircuitError>;
}

/// Constraint satisfaction test
pub struct ConstraintSatisfactionTest {
    name: String,
    circuit: Box<dyn Fn() -> Result<ConstraintSystem, CircuitError>>,
}

impl ConstraintSatisfactionTest {
    pub fn new(
        name: impl Into<String>,
        circuit: impl Fn() -> Result<ConstraintSystem, CircuitError> + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            circuit: Box::new(circuit),
        }
    }
}

impl CircuitTest for ConstraintSatisfactionTest {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> Result<(), CircuitError> {
        let cs = (self.circuit)()?;

        if !cs.is_satisfied() {
            return Err(CircuitError::ConstraintNotSatisfied(
                "Constraints not satisfied".to_string(),
            ));
        }

        Ok(())
    }
}

/// Property-based test
pub struct PropertyTest {
    name: String,
    num_iterations: usize,
    property: Box<dyn Fn() -> Result<(), CircuitError>>,
}

impl PropertyTest {
    pub fn new(
        name: impl Into<String>,
        num_iterations: usize,
        property: impl Fn() -> Result<(), CircuitError> + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            num_iterations,
            property: Box::new(property),
        }
    }
}

impl CircuitTest for PropertyTest {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> Result<(), CircuitError> {
        for i in 0..self.num_iterations {
            (self.property)().map_err(|e| {
                CircuitError::InvalidInput(format!("Property failed at iteration {}: {}", i, e))
            })?;
        }

        Ok(())
    }
}

/// Fuzzing test
pub struct FuzzTest {
    name: String,
    duration: Duration,
    fuzzer: Box<dyn Fn() -> Result<(), CircuitError>>,
}

impl FuzzTest {
    pub fn new(
        name: impl Into<String>,
        duration: Duration,
        fuzzer: impl Fn() -> Result<(), CircuitError> + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            duration,
            fuzzer: Box::new(fuzzer),
        }
    }
}

impl CircuitTest for FuzzTest {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> Result<(), CircuitError> {
        let start = Instant::now();
        let mut iterations = 0;

        while start.elapsed() < self.duration {
            (self.fuzzer)().map_err(|e| {
                CircuitError::InvalidInput(format!("Fuzz test failed at iteration {}: {}", iterations, e))
            })?;
            iterations += 1;
        }

        println!("    Fuzz test completed {} iterations", iterations);

        Ok(())
    }
}

/// Performance test
pub struct PerformanceTest {
    name: String,
    max_duration: Duration,
    operation: Box<dyn Fn() -> Result<(), CircuitError>>,
}

impl PerformanceTest {
    pub fn new(
        name: impl Into<String>,
        max_duration: Duration,
        operation: impl Fn() -> Result<(), CircuitError> + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            max_duration,
            operation: Box::new(operation),
        }
    }
}

impl CircuitTest for PerformanceTest {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> Result<(), CircuitError> {
        let start = Instant::now();
        (self.operation)()?;
        let elapsed = start.elapsed();

        if elapsed > self.max_duration {
            return Err(CircuitError::CompilationError(format!(
                "Performance test exceeded max duration: {:?} > {:?}",
                elapsed, self.max_duration
            )));
        }

        println!("    Completed in {:?}", elapsed);

        Ok(())
    }
}

/// Test builder for fluent API
pub struct TestBuilder {
    suite: TestSuite,
}

impl TestBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            suite: TestSuite::new(name),
        }
    }

    pub fn test_constraint_satisfaction(
        mut self,
        name: impl Into<String>,
        circuit: impl Fn() -> Result<ConstraintSystem, CircuitError> + 'static,
    ) -> Self {
        self.suite.add_test(Box::new(ConstraintSatisfactionTest::new(name, circuit)));
        self
    }

    pub fn test_property(
        mut self,
        name: impl Into<String>,
        iterations: usize,
        property: impl Fn() -> Result<(), CircuitError> + 'static,
    ) -> Self {
        self.suite.add_test(Box::new(PropertyTest::new(name, iterations, property)));
        self
    }

    pub fn test_fuzz(
        mut self,
        name: impl Into<String>,
        duration: Duration,
        fuzzer: impl Fn() -> Result<(), CircuitError> + 'static,
    ) -> Self {
        self.suite.add_test(Box::new(FuzzTest::new(name, duration, fuzzer)));
        self
    }

    pub fn test_performance(
        mut self,
        name: impl Into<String>,
        max_duration: Duration,
        operation: impl Fn() -> Result<(), CircuitError> + 'static,
    ) -> Self {
        self.suite.add_test(Box::new(PerformanceTest::new(name, max_duration, operation)));
        self
    }

    pub fn build(self) -> TestSuite {
        self.suite
    }

    pub fn run(mut self) -> TestReport {
        self.suite.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::circuit::primitives::*;

    #[test]
    fn test_constraint_satisfaction_test() {
        let mut suite = TestSuite::new("Basic Tests");

        suite.add_test(Box::new(ConstraintSatisfactionTest::new(
            "multiplication",
            || {
                let mut cs = ConstraintSystem::new();
                let a = cs.alloc_variable(Some(Field::from(3)));
                let b = cs.alloc_variable(Some(Field::from(4)));
                let c = cs.alloc_variable(Some(Field::from(12)));
                cs.enforce_mul(a, b, c);
                Ok(cs)
            },
        )));

        let report = suite.run();
        assert!(report.is_success());
        assert_eq!(report.passed, 1);
    }

    #[test]
    fn test_property_test() {
        let mut suite = TestSuite::new("Property Tests");

        suite.add_test(Box::new(PropertyTest::new(
            "addition_commutative",
            100,
            || {
                use ark_ff::Field as ArkField;
                let a = Field::from(rand::random::<u32>());
                let b = Field::from(rand::random::<u32>());

                if a + b != b + a {
                    return Err(CircuitError::InvalidInput("Addition not commutative".to_string()));
                }

                Ok(())
            },
        )));

        let report = suite.run();
        assert!(report.is_success());
    }

    #[test]
    fn test_builder_api() {
        let report = TestBuilder::new("Builder API Tests")
            .test_constraint_satisfaction("simple_mul", || {
                let mut cs = ConstraintSystem::new();
                let a = cs.alloc_variable(Some(Field::from(2)));
                let b = cs.alloc_variable(Some(Field::from(3)));
                let c = cs.alloc_variable(Some(Field::from(6)));
                cs.enforce_mul(a, b, c);
                Ok(cs)
            })
            .test_performance(
                "allocation_performance",
                Duration::from_millis(100),
                || {
                    let mut cs = ConstraintSystem::new();
                    for i in 0..1000 {
                        cs.alloc_variable(Some(Field::from(i)));
                    }
                    Ok(())
                },
            )
            .run();

        assert!(report.is_success());
        assert_eq!(report.total, 2);
    }
}
