use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_184473: FileType = FileType {
    file_format: &FileFormat {
        id: 184_473,
        source_type: SourceType::Wikidata,
        name: "OpenDocument",
        extensions: &["fodt", "odt"],
        media_types: &["application/vnd.oasis.opendocument.text"],
        signatures: &[],
        related_formats: &[],
    },
};
