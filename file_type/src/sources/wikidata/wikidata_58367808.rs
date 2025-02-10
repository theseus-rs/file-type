use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58367808: FileType = FileType {
    file_format: &FileFormat {
        id: 58_367_808,
        source_type: SourceType::Wikidata,
        name: "BSDIFF",
        extensions: &["bsdiff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
