use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206310: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_310,
        source_type: SourceType::Wikidata,
        name: "Analyze HDR",
        extensions: &["hdr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
