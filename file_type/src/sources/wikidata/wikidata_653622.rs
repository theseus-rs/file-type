use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_653622: FileType = FileType {
    file_format: &FileFormat {
        id: 653_622,
        source_type: SourceType::Wikidata,
        name: "SILK",
        extensions: &["SIL", "sil"],
        media_types: &["audio/SILK"],
        signatures: &[],
        related_formats: &[],
    },
};
