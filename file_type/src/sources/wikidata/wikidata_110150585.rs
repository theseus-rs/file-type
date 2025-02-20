use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110150585: FileType = FileType {
    file_format: &FileFormat {
        id: 110_150_585,
        source_type: SourceType::Wikidata,
        name: "Serif DrawPlus Drawing, version 7",
        extensions: &["dpa", "dpp", "dpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
