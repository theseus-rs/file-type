use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_161: FileFormat = FileFormat {
    id: 161,
    source_type: SourceType::Linguist,
    name: "IDL",
    extensions: &["dlm", "pro"],
    media_types: &["text/x-idl"],
    internal_signatures: &[],
    related_formats: &[],
};
