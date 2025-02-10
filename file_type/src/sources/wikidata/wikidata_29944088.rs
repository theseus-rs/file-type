use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29944088: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_088,
        source_type: SourceType::Wikidata,
        name: "Sun XML Writer",
        extensions: &["sxw"],
        media_types: &["application/vnd.sun.xml.writer"],
        signatures: &[],
        related_formats: &[],
    },
};
