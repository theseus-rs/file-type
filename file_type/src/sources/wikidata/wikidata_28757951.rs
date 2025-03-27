use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757951: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_951,
        source_type: SourceType::Wikidata,
        name: "HDV (disk image)",
        extensions: &["hdv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
