use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_24907733: FileType = FileType {
    file_format: &FileFormat {
        id: 24_907_733,
        source_type: SourceType::Wikidata,
        name: "High Efficiency Image File Format",
        extensions: &["heic", "heif"],
        media_types: &[
            "image/heic",
            "image/heic-sequence",
            "image/heif",
            "image/heif-sequence",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
