use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363609: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_609,
        source_type: SourceType::Wikidata,
        name: "Westacott WinRanX instrument file",
        extensions: &["wrf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
