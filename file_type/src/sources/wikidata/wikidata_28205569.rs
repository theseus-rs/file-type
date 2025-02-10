use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205569: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_569,
        source_type: SourceType::Wikidata,
        name: "Nokia Startup Logo",
        extensions: &["nsl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
