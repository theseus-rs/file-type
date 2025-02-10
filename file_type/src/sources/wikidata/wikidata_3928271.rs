use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3928271: FileType = FileType {
    file_format: &FileFormat {
        id: 3_928_271,
        source_type: SourceType::Wikidata,
        name: "RGBE image format",
        extensions: &["hdr", "pic", "rad", "rgbe", "xyze"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
