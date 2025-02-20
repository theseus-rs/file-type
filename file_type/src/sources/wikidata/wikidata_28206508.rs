use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206508: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_508,
        source_type: SourceType::Wikidata,
        name: "Light Sheet Microscope",
        extensions: &["lsm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
