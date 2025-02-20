use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125926524: FileType = FileType {
    file_format: &FileFormat {
        id: 125_926_524,
        source_type: SourceType::Wikidata,
        name: "Solidworks Drawing Sheet",
        extensions: &["slddrt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
