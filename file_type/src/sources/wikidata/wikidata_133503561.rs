use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133503561: FileType = FileType {
    file_format: &FileFormat {
        id: 133_503_561,
        source_type: SourceType::Wikidata,
        name: "Dali high resolution file",
        extensions: &["hpk", "sd2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
