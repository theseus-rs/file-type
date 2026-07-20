use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_850954: FileType = FileType {
    file_format: &FileFormat {
        id: 850_954,
        source_type: SourceType::Wikidata,
        name: "Motion JPEG",
        extensions: &["mjpeg", "mjpg"],
        media_types: &["video/x-mjpeg"],
        signatures: &[],
        related_formats: &[],
    },
};
