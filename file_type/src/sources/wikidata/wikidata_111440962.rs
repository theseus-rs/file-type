use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111440962: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_962,
        source_type: SourceType::Wikidata,
        name: "Visual Basic UserControl Object File",
        extensions: &["ctl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
