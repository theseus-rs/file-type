use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
