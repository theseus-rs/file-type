use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134272221: FileType = FileType {
    file_format: &FileFormat {
        id: 134_272_221,
        source_type: SourceType::Wikidata,
        name: "Clipper source program file",
        extensions: &["prg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
