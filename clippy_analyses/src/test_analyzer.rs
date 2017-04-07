use rustc::lint::*;

use syntax::ast;
use syntax::codemap::{Span};
use syntax::visit::FnKind;

use datalog::predicates::*;
use datalog::writer::*;

declare_lint! {
    pub TEST_ANALYZER,
    Allow,
    "Tests whether this analysis setup will work"
}

// #[derive(Copy, Clone)]
pub struct TestAnalyzer<T> where T: Writer {
    writer: Box<T>,
}

impl <T> LintPass for TestAnalyzer<T> where T: Writer {
    fn get_lints(&self) -> LintArray {
        lint_array![TEST_ANALYZER]
    }
}

impl <'a,T> EarlyLintPass for TestAnalyzer<T> where T: Writer {
    fn check_fn(&mut self, _: &EarlyContext, kind: FnKind, _: &ast::FnDecl, _: Span, _: ast::NodeId) {
        match kind {
            FnKind::ItemFn(ref ident, ..) => {
                let name = &*ident.name.as_str();
                self.writer.add(Function(&name)).unwrap();
            },
            FnKind::Method(ref ident, ..) => {
                let name = &*ident.name.as_str();
                self.writer.add(Function(&name)).unwrap();
            },
            FnKind::Closure(..) => (),
        }
    }
}

impl <'a,T> TestAnalyzer<T> where T: Writer {
    pub fn new(writer: T) -> TestAnalyzer<T> {
        TestAnalyzer{ writer: box writer }
    }
}

fn Function(name: &str) -> Stmt {
        Stmt::Fact(Pred::new(
            "function",
            vec![ID::Literal(name)]))
}
