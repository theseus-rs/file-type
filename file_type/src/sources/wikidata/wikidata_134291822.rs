use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134291822: FileType = FileType {
    file_format: &FileFormat {
        id: 134_291_822,
        source_type: SourceType::Wikidata,
        name: "Clipper reports definition file",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
