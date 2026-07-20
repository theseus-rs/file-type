use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138358440: FileType = FileType {
    file_format: &FileFormat {
        id: 138_358_440,
        source_type: SourceType::Wikidata,
        name: "MRtrix File Format",
        extensions: &["mif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
