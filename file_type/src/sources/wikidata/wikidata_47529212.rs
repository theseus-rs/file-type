use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47529212: FileType = FileType {
    file_format: &FileFormat {
        id: 47_529_212,
        source_type: SourceType::Wikidata,
        name: "VLW font file",
        extensions: &["vlw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
