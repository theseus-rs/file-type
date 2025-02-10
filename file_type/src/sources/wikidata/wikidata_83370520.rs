use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83370520: FileType = FileType {
    file_format: &FileFormat {
        id: 83_370_520,
        source_type: SourceType::Wikidata,
        name: "Midi Sample dump Format",
        extensions: &["sds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
