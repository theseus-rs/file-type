use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4645195: FileType = FileType {
    file_format: &FileFormat {
        id: 4_645_195,
        source_type: SourceType::Wikidata,
        name: "8-Bit Sampled Voice",
        extensions: &["8svx", "iff"],
        media_types: &["audio/8svx", "audio/x-8svx"],
        signatures: &[],
        related_formats: &[],
    },
};
