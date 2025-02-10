use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3850662786: FileType = FileType {
    file_format: &FileFormat {
        id: 3_850_662_786,
        source_type: SourceType::Iana,
        name: "vnd.netfpx",
        extensions: &[],
        media_types: &["application/vnd.netfpx"],
        signatures: &[],
        related_formats: &[],
    },
};
