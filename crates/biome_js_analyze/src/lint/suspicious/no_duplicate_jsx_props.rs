use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{AnyJsxAttribute, JsxAttribute};
use biome_rowan::AstNode;
use biome_rule_options::no_duplicate_jsx_props::NoDuplicateJsxPropsOptions;
use rustc_hash::FxHashMap;

declare_lint_rule! {
    /// Prevents JSX properties to be assigned multiple times.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Hello name="John" name="John" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label xml:lang="en-US" xml:lang="en-US"></label>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Hello firstname="John" lastname="Doe" />
    /// ```
    ///
    /// ```jsx
    /// <label xml:lang="en-US" lang="en-US"></label>
    /// ```
 pub NoDuplicateJsxProps {
        version: "1.0.0",
        name: "noDuplicateJsxProps",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-no-duplicate-props").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoDuplicateJsxProps {
    type Query = Ast<AnyJsxElement>;
    type State = (Box<str>, Vec<JsxAttribute>);
    type Signals = FxHashMap<Box<str>, Vec<JsxAttribute>>;
    type Options = NoDuplicateJsxPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_attributes: FxHashMap<Box<str>, Vec<JsxAttribute>> = FxHashMap::default();
        for attribute in node.attributes() {
            if let AnyJsxAttribute::JsxAttribute(attr) = attribute {
                if let Ok(name) = attr.name() {
                    defined_attributes
                        .entry(name.to_trimmed_text().text().into())
                        .or_default()
                        .push(attr);
                }
            }
        }

        defined_attributes.retain(|_, val| val.len() > 1);
        defined_attributes
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut attributes = state.1.iter();

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            attributes.next()?.syntax().text_trimmed_range(),
            markup!("This JSX property is assigned multiple times."),
        );

        for attr in attributes {
            diagnostic = diagnostic.detail(
                attr.syntax().text_trimmed_range(),
                "This attribute is assigned again here.",
            )
        }

        Some(diagnostic)
    }
}
