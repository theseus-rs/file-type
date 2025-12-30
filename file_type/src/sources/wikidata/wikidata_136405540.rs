use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136405540: FileType = FileType {
    file_format: &FileFormat {
        id: 136_405_540,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Text, version 1.4",
        extensions: &["odt"],
        media_types: &["application/vnd.oasis.opendocument.text"],
        signatures: &[],
        related_formats: &[],
    },
};
