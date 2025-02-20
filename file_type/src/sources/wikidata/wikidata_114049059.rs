use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114049059: FileType = FileType {
    file_format: &FileFormat {
        id: 114_049_059,
        source_type: SourceType::Wikidata,
        name: "Rocky Interlace Picture",
        extensions: &["rip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
