use oxc_ast::{ast::Expression, AstKind};

#[allow(unused_imports)]
use oxc_diagnostics::{
    miette::{self, Diagnostic},
    thiserror::{self, Error},
};
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{context::LintContext, rule::Rule, AstNode};

#[derive(Debug, Error, Diagnostic)]
#[error("eslint(ubugeeei-sample):")]
#[diagnostic(severity(warning), help(""))]
struct UbugeeeiSampleDiagnostic(#[label] pub Span);

#[derive(Debug, Default, Clone)]
pub struct UbugeeeiSample;

declare_oxc_lint!(
    /// ### What it does
    ///
    ///
    /// ### Why is this bad?
    ///
    ///
    /// ### Example
    /// ```javascript
    /// ```
    UbugeeeiSample,
    correctness
);

impl Rule for UbugeeeiSample {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        if let AstKind::ExpressionStatement(expr) = node.kind() {
            if let Expression::Identifier(ident) = &expr.expression {
                if ident.name == "ubugeeei" {
                    ctx.diagnostic(UbugeeeiSampleDiagnostic(expr.span));
                }
            }
        }
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![("a;", None)];

    let fail = vec![("ubugeeei;", None)];

    Tester::new(UbugeeeiSample::NAME, pass, fail).test_and_snapshot();
}
