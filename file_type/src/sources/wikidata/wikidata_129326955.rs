use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129326955: FileType = FileType {
    file_format: &FileFormat {
        id: 129_326_955,
        source_type: SourceType::Wikidata,
        name: "GDScript source code file",
        extensions: &["gd"],
        media_types: &["application/x-gdscript", "text/x-gdscript"],
        signatures: &[],
        related_formats: &[],
    },
};
