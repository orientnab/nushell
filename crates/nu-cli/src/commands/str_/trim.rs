use crate::commands::WholeStreamCommand;
use crate::prelude::*;
use nu_errors::ShellError;
use nu_protocol::ShellTypeName;
use nu_protocol::{
    ColumnPath, Primitive, ReturnSuccess, Signature, SyntaxShape, UntaggedValue, Value,
};
use nu_source::Tag;
use nu_value_ext::ValueExt;

#[derive(Deserialize)]
struct Arguments {
    rest: Vec<ColumnPath>,
}

pub struct SubCommand;

#[async_trait]
impl WholeStreamCommand for SubCommand {
    fn name(&self) -> &str {
        "str trim"
    }

    fn signature(&self) -> Signature {
        Signature::build("str trim").rest(
            SyntaxShape::ColumnPath,
            "optionally trim text by column paths",
        )
    }

    fn usage(&self) -> &str {
        "trims text"
    }

    async fn run(
        &self,
        args: CommandArgs,
        registry: &CommandRegistry,
    ) -> Result<OutputStream, ShellError> {
        operate(args, registry).await
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Trim contents",
            example: "echo 'Nu shell ' | str trim",
            result: Some(vec![Value::from("Nu shell")]),
        }]
    }
}

async fn operate(
    args: CommandArgs,
    registry: &CommandRegistry,
) -> Result<OutputStream, ShellError> {
    let registry = registry.clone();

    let (Arguments { rest }, input) = args.process(&registry).await?;

    let column_paths: Vec<_> = rest;

    Ok(input
        .map(move |v| {
            if column_paths.is_empty() {
                match action(&v, v.tag()) {
                    Ok(out) => ReturnSuccess::value(out),
                    Err(err) => Err(err),
                }
            } else {
                let mut ret = v;

                for path in &column_paths {
                    let swapping = ret.swap_data_by_column_path(
                        path,
                        Box::new(move |old| action(old, old.tag())),
                    );

                    match swapping {
                        Ok(new_value) => {
                            ret = new_value;
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }

                ReturnSuccess::value(ret)
            }
        })
        .to_output_stream())
}

fn action(input: &Value, tag: impl Into<Tag>) -> Result<Value, ShellError> {
    match &input.value {
        UntaggedValue::Primitive(Primitive::Line(s))
        | UntaggedValue::Primitive(Primitive::String(s)) => {
            Ok(UntaggedValue::string(s.trim()).into_value(tag))
        }
        other => {
            let got = format!("got {}", other.type_name());
            Err(ShellError::labeled_error(
                "value is not string",
                got,
                tag.into().span,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{action, SubCommand};
    use nu_plugin::test_helpers::value::string;
    use nu_source::Tag;

    #[test]
    fn examples_work_as_expected() {
        use crate::examples::test as test_examples;

        test_examples(SubCommand {})
    }

    #[test]
    fn trims() {
        let word = string("andres ");
        let expected = string("andres");

        let actual = action(&word, Tag::unknown()).unwrap();
        assert_eq!(actual, expected);
    }
}
