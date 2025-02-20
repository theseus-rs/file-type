use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125297151: FileType = FileType {
    file_format: &FileFormat {
        id: 125_297_151,
        source_type: SourceType::Wikidata,
        name: "cdb format",
        extensions: &["cdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
