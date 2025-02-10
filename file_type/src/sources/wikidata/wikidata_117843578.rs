use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117843578: FileType = FileType {
    file_format: &FileFormat {
        id: 117_843_578,
        source_type: SourceType::Wikidata,
        name: "Faxable TIF",
        extensions: &["ftf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
