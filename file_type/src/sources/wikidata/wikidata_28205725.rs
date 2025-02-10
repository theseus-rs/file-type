use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205725: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_725,
        source_type: SourceType::Wikidata,
        name: "Async Professional Fax",
        extensions: &["apf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
