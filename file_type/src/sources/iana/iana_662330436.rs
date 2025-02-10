use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_662330436: FileType = FileType {
    file_format: &FileFormat {
        id: 662_330_436,
        source_type: SourceType::Iana,
        name: "EVRCB0",
        extensions: &[],
        media_types: &["audio/EVRCB0"],
        signatures: &[],
        related_formats: &[],
    },
};
