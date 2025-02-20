use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110152549: FileType = FileType {
    file_format: &FileFormat {
        id: 110_152_549,
        source_type: SourceType::Wikidata,
        name: "Serif DrawPlus Drawing, version X6",
        extensions: &["dpa", "dpp", "dpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
