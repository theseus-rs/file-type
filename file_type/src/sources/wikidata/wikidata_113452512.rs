use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113452512: FileType = FileType {
    file_format: &FileFormat {
        id: 113_452_512,
        source_type: SourceType::Wikidata,
        name: "PageMaker Mac Document Version 4",
        extensions: &[],
        media_types: &["application/vnd.pagemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
