use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59630317: FileType = FileType {
    file_format: &FileFormat {
        id: 59_630_317,
        source_type: SourceType::Wikidata,
        name: "P00 C64 Image Format",
        extensions: &["p00", "p01", "p02", "p03", "p04"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
