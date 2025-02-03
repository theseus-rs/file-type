use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_48: FileFormat = FileFormat {
    id: 48,
    source_type: SourceType::Linguist,
    name: "COBOL",
    extensions: &["cbl", "ccp", "cob", "cobol", "cpy"],
    media_types: &["text/x-cobol"],
    internal_signatures: &[],
    related_formats: &[],
};
