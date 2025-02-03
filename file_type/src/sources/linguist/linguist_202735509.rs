use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_202735509: FileFormat = FileFormat {
    id: 202_735_509,
    source_type: SourceType::Linguist,
    name: "ObjectScript",
    extensions: &["cls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
