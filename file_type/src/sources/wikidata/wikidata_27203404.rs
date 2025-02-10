use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27203404: FileType = FileType {
    file_format: &FileFormat {
        id: 27_203_404,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Text, version 1.1",
        extensions: &["odt"],
        media_types: &["application/vnd.oasis.opendocument.text"],
        signatures: &[],
        related_formats: &[],
    },
};
