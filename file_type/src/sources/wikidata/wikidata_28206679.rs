use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206679: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_679,
        source_type: SourceType::Wikidata,
        name: "Portable Bitmap, binary variant",
        extensions: &["pbm", "pnm"],
        media_types: &["image/x-portable-anymap", "image/x-portable-bitmap"],
        signatures: &[],
        related_formats: &[],
    },
};
