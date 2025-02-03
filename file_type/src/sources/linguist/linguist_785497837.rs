use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_785497837: FileFormat = FileFormat {
    id: 785_497_837,
    source_type: SourceType::Linguist,
    name: "Avro IDL",
    extensions: &["avdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
