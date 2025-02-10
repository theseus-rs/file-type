use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124080600: FileType = FileType {
    file_format: &FileFormat {
        id: 124_080_600,
        source_type: SourceType::Wikidata,
        name: "CSL-JSON",
        extensions: &["json"],
        media_types: &["application/vnd.citationstyles.csl+json"],
        signatures: &[],
        related_formats: &[],
    },
};
