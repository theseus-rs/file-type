use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50825837: FileType = FileType {
    file_format: &FileFormat {
        id: 50_825_837,
        source_type: SourceType::Wikidata,
        name: "AVCHD Movie Object File",
        extensions: &["bdm", "bdmv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
