use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_50376365: FileType = FileType {
    file_format: &FileFormat {
        id: 50_376_365,
        source_type: SourceType::Wikidata,
        name: "File Geodatabase (Esri)",
        extensions: &["gdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
