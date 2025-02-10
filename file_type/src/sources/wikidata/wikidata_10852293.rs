use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_10852293: FileType = FileType {
    file_format: &FileFormat {
        id: 10_852_293,
        source_type: SourceType::Wikidata,
        name: "RPT",
        extensions: &["rpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
