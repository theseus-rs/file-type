use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51917759: FileType = FileType {
    file_format: &FileFormat {
        id: 51_917_759,
        source_type: SourceType::Wikidata,
        name: "SDSC Image Tool X Window Dump Format",
        extensions: &["xwd"],
        media_types: &["image/xwd"],
        signatures: &[],
        related_formats: &[],
    },
};
