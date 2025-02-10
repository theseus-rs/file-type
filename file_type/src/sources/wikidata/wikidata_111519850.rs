use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111519850: FileType = FileType {
    file_format: &FileFormat {
        id: 111_519_850,
        source_type: SourceType::Wikidata,
        name: "Stata .do command file",
        extensions: &["do"],
        media_types: &["application/x-stata", "text/stata", "text/x-stata"],
        signatures: &[],
        related_formats: &[],
    },
};
