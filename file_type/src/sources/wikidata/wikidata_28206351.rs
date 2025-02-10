use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206351: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_351,
        source_type: SourceType::Wikidata,
        name: "Inset PIX",
        extensions: &["pix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
