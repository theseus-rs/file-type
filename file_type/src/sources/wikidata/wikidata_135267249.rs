use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135267249: FileType = FileType {
    file_format: &FileFormat {
        id: 135_267_249,
        source_type: SourceType::Wikidata,
        name: "FTLX file",
        extensions: &["ftlx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
