use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110150521: FileType = FileType {
    file_format: &FileFormat {
        id: 110_150_521,
        source_type: SourceType::Wikidata,
        name: "Serif DrawPlus Drawing, version 6",
        extensions: &["dpa", "dpp", "dpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
