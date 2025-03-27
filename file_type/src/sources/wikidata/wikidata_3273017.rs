use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3273017: FileType = FileType {
    file_format: &FileFormat {
        id: 3_273_017,
        source_type: SourceType::Wikidata,
        name: "Motion JPEG 2000",
        extensions: &["mj2", "mjp2"],
        media_types: &["video/mj2"],
        signatures: &[],
        related_formats: &[],
    },
};
