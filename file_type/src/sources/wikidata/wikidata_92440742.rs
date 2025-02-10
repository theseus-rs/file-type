use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_92440742: FileType = FileType {
    file_format: &FileFormat {
        id: 92_440_742,
        source_type: SourceType::Wikidata,
        name: "Spider 2D image",
        extensions: &["spider"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
