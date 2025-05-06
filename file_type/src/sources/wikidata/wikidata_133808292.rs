use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133808292: FileType = FileType {
    file_format: &FileFormat {
        id: 133_808_292,
        source_type: SourceType::Wikidata,
        name: "MLDF file",
        extensions: &["mldf"],
        media_types: &["image/x-mldf"],
        signatures: &[],
        related_formats: &[],
    },
};
