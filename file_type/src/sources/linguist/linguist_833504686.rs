use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_833504686: FileFormat = FileFormat {
    id: 833_504_686,
    source_type: SourceType::Linguist,
    name: "PlantUML",
    extensions: &["iuml", "plantuml", "puml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
