use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116784985: FileType = FileType {
    file_format: &FileFormat {
        id: 116_784_985,
        source_type: SourceType::Wikidata,
        name: "Project Manager Pro Chart file",
        extensions: &["pmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
