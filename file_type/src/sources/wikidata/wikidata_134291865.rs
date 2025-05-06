use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134291865: FileType = FileType {
    file_format: &FileFormat {
        id: 134_291_865,
        source_type: SourceType::Wikidata,
        name: "Clipper format file",
        extensions: &["fmt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
