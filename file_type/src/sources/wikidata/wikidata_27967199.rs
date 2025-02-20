use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967199: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_199,
        source_type: SourceType::Wikidata,
        name: "Liquid Tracker module",
        extensions: &["liq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
