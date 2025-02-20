use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207212: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_212,
        source_type: SourceType::Wikidata,
        name: "Quantel VPB image",
        extensions: &["vpb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
