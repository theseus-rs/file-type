use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59913607: FileType = FileType {
    file_format: &FileFormat {
        id: 59_913_607,
        source_type: SourceType::Wikidata,
        name: "AV1 Image File Format",
        extensions: &["avif"],
        media_types: &["image/avif"],
        signatures: &[],
        related_formats: &[],
    },
};
