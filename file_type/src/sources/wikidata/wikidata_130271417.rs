use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130271417: FileType = FileType {
    file_format: &FileFormat {
        id: 130_271_417,
        source_type: SourceType::Wikidata,
        name: "Mako file format",
        extensions: &["mao"],
        media_types: &["application/x-mako"],
        signatures: &[],
        related_formats: &[],
    },
};
