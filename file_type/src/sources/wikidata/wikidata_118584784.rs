use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118584784: FileType = FileType {
    file_format: &FileFormat {
        id: 118_584_784,
        source_type: SourceType::Wikidata,
        name: "Cakewalk Bundle",
        extensions: &["cwb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
