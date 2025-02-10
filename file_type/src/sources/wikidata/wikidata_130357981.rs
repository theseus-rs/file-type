use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130357981: FileType = FileType {
    file_format: &FileFormat {
        id: 130_357_981,
        source_type: SourceType::Wikidata,
        name: "MoonScript source code file",
        extensions: &["moon"],
        media_types: &["application/x-moonscript", "text/x-moonscript"],
        signatures: &[],
        related_formats: &[],
    },
};
