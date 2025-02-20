use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_288256: FileType = FileType {
    file_format: &FileFormat {
        id: 288_256,
        source_type: SourceType::Wikidata,
        name: "ACE",
        extensions: &["ace"],
        media_types: &["application/x-ace-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
