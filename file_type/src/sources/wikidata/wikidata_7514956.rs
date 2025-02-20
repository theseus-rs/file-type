use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7514956: FileType = FileType {
    file_format: &FileFormat {
        id: 7_514_956,
        source_type: SourceType::Wikidata,
        name: "Silicon Graphics Image",
        extensions: &["bw", "rgb", "rgba", "sgi"],
        media_types: &["image/sgi", "image/x-rgb", "image/x-sgi", "image/x-sgi-rgb"],
        signatures: &[],
        related_formats: &[],
    },
};
