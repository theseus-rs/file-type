use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110238151: FileType = FileType {
    file_format: &FileFormat {
        id: 110_238_151,
        source_type: SourceType::Wikidata,
        name: "SunRF",
        extensions: &["rf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
