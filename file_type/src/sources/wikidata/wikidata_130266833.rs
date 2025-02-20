use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130266833: FileType = FileType {
    file_format: &FileFormat {
        id: 130_266_833,
        source_type: SourceType::Wikidata,
        name: "Macaulay2 file format",
        extensions: &["m2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
