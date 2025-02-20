use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50564741: FileType = FileType {
    file_format: &FileFormat {
        id: 50_564_741,
        source_type: SourceType::Wikidata,
        name: "AVCHD Clip Information File",
        extensions: &["clpi", "cpi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
