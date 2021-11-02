use anyhow::Result;
use iai::black_box;

use xq::parser;

fn benchmark_parse_enqueue_int() -> Result<()> {
    parser::parse("enqueue a 1")?;
    Ok(())
}

fn benchmark_parse_enqueue_float() -> Result<()> {
    parser::parse("enqueue a 1.01")?;
    Ok(())
}

fn benchmark_parse_enqueue_string() -> Result<()> {
    parser::parse("enqueue a \"this is a thing\"")?;
    Ok(())
}

fn benchmark_parse_dequeue() -> Result<()> {
    parser::parse("dequeue a")?;
    Ok(())
}

fn benchmark_parse_peek() -> Result<()> {
    parser::parse("peek a")?;
    Ok(())
}

fn benchmark_parse_length() -> Result<()> {
    parser::parse("length a")?;
    Ok(())
}

fn benchmark_parse_full_program() -> Result<()> {
    parser::parse("
        enqueue a 1
        enqueue a 1.0
        enqueue a 1.0001
        enqueue a 1.2
        enqueue a \"1\"
        enqueue a \"omg\"
        enqueue a 1
        enqueue a 1
        enqueue a 1
        enqueue a 1
        enqueue a 1
        enqueue a 1
        dequeue a
        peek a
        peek a
        dequeue a
        peek a
    ")?;

    Ok(())
}

iai::main!(
    benchmark_parse_enqueue_int,
    benchmark_parse_enqueue_float,
    benchmark_parse_enqueue_string,
    benchmark_parse_dequeue,
    benchmark_parse_peek,
    benchmark_parse_length,
    benchmark_parse_full_program
);
