use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111272661: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_661,
        source_type: SourceType::Wikidata,
        name: "Ensoniq EPS family compacted disk image",
        extensions: &["eui"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
