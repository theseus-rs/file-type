use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139813099: FileType = FileType {
    file_format: &FileFormat {
        id: 139_813_099,
        source_type: SourceType::Wikidata,
        name: "Graphics Filter",
        extensions: &["flt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
