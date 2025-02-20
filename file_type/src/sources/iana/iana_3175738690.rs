use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3175738690: FileType = FileType {
    file_format: &FileFormat {
        id: 3_175_738_690,
        source_type: SourceType::Iana,
        name: "vnd.cups-raster",
        extensions: &[],
        media_types: &["application/vnd.cups-raster"],
        signatures: &[],
        related_formats: &[],
    },
};
