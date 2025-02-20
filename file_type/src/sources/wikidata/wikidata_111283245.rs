use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111283245: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_245,
        source_type: SourceType::Wikidata,
        name: "Floating point raw 64-bit IEEE data",
        extensions: &["f64"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
