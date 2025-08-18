use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135267202: FileType = FileType {
    file_format: &FileFormat {
        id: 135_267_202,
        source_type: SourceType::Wikidata,
        name: "FTLH file",
        extensions: &["ftlh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
