use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2389386: FileType = FileType {
    file_format: &FileFormat {
        id: 2_389_386,
        source_type: SourceType::Wikidata,
        name: "Interleaved Bitmap",
        extensions: &["bbm", "iff", "ilbm", "lbm", "pic"],
        media_types: &["image/x-ilbm"],
        signatures: &[],
        related_formats: &[],
    },
};
