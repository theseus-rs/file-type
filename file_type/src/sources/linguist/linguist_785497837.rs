use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_785497837: FileType = FileType {
    file_format: &FileFormat {
        id: 785_497_837,
        source_type: SourceType::Linguist,
        name: "Avro IDL",
        extensions: &["avdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
