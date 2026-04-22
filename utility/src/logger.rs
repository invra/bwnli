use std::fmt;
use tracing::Level;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id, Record};
use tracing::{Event, Metadata, Subscriber};

pub struct SimpleSubscriber;

struct FieldVisitor {
    message: Option<String>,
    title: Option<String>,
    description: Option<String>,
    fields: Vec<(String, String)>,
}

impl FieldVisitor {
    fn new() -> Self {
        Self {
            message: None,
            title: None,
            description: None,
            fields: Vec::new(),
        }
    }
}

impl Visit for FieldVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "title" => self.title = Some(value.to_string()),
            "description" => self.description = Some(value.to_string()),
            _ => self
                .fields
                .push((field.name().to_string(), value.to_string())),
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        match field.name() {
            "message" => self.message = Some(format!("{value:?}")),
            _ => self
                .fields
                .push((field.name().to_string(), format!("{value:?}"))),
        }
    }
}

const MAX_LEVEL: Level = Level::DEBUG;

fn level_color(level: &Level) -> &'static str {
    match level {
        &Level::TRACE => "\x1b[94m",
        &Level::DEBUG => "\x1b[34m",
        &Level::INFO => "\x1b[32m",
        &Level::WARN => "\x1b[33m",
        &Level::ERROR => "\x1b[31m",
    }
}

fn level_injection(level: &Level) -> String {
    let color = level_color(level);
    let level_str = match level {
        &Level::TRACE => "trace",
        &Level::DEBUG => "debug",
        &Level::INFO => "info",
        &Level::WARN => "warning",
        &Level::ERROR => "error",
    };
    format!("{color}{level_str}\x1b[0m")
}

impl Subscriber for SimpleSubscriber {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        *metadata.level() <= MAX_LEVEL
    }

    fn new_span(&self, _attrs: &Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let meta = event.metadata();
        let mut visitor = FieldVisitor::new();
        event.record(&mut visitor);

        let level = meta.level();
        let target = meta.target();
        let file = meta.file().unwrap_or("unknown_file.rs");
        let line = meta.line().unwrap_or(0);
        let prefix = level_injection(level);

        match (visitor.title, visitor.description) {
            (Some(title), Some(description)) => {
                println!("\x1b[1m{prefix}\x1b[1m: {title}\x1b[0m");
                println!(" \x1b[34;1m-->\x1b[0m {target}::{file}:{line}");
                println!("  \x1b[34;1m|\x1b[0m");
                for line in description.lines() {
                    let treated_line = line.trim_start();
                    println!("  \x1b[34;1m|\x1b[0m {treated_line}");
                }
                println!("  \x1b[34;1m|\x1b[0m");
                println!("  \x1b[34;1m=\x1b[0m");
            }
            (Some(title), None) => {
                println!("\x1b[1m{prefix}: {target} -> {title}");
            }
            _ => {
                let message = visitor.message.unwrap_or_default();
                println!("\x1b[1m{prefix}: {target} -> {message}");
                for (k, v) in visitor.fields {
                    print!(" {k}={v}");
                }
            }
        }
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}
