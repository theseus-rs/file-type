use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_68481410: FileType = FileType {
    file_format: &FileFormat {
        id: 68_481_410,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Sheet Set file format",
        extensions: &["dst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
