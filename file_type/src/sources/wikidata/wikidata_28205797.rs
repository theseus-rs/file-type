use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205797: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_797,
        source_type: SourceType::Wikidata,
        name: "Canvas Picture Format",
        extensions: &["cnv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
