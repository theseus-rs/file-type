use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_341: FileFormat = FileFormat {
    id: 341,
    source_type: SourceType::Linguist,
    name: "Scala",
    extensions: &["kojo", "sbt", "sc", "scala"],
    media_types: &["text/x-scala"],
    internal_signatures: &[],
    related_formats: &[],
};
