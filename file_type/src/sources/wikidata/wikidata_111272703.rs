use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272703: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_703,
        source_type: SourceType::Wikidata,
        name: "Floating Point raw 32-bit IEEE data",
        extensions: &["f32"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
