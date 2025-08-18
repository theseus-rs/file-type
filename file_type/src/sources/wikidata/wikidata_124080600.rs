use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124080600: FileType = FileType {
    file_format: &FileFormat {
        id: 124_080_600,
        source_type: SourceType::Wikidata,
        name: "Citation Style Language JSON",
        extensions: &["json"],
        media_types: &[
            "application/json",
            "application/vnd.citationstyles.csl+json",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
