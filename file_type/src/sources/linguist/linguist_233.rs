use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_233: FileFormat = FileFormat {
    id: 233,
    source_type: SourceType::Linguist,
    name: "Modelica",
    extensions: &["mo"],
    media_types: &["text/x-modelica"],
    signatures: &[],
    related_formats: &[],
};
