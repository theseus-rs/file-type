use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117224387: FileType = FileType {
    file_format: &FileFormat {
        id: 117_224_387,
        source_type: SourceType::Wikidata,
        name: "TurboCAD for DOS 3.0 file",
        extensions: &["tcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
