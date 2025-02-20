use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363704: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_704,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif XF 'all' format",
        extensions: &["x3a"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
