use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2381992798: FileType = FileType {
    file_format: &FileFormat {
        id: 2_381_992_798,
        source_type: SourceType::Iana,
        name: "vnd.veraison.nvidia-gpu-evidence+json",
        extensions: &[],
        media_types: &["application/vnd.veraison.nvidia-gpu-evidence+json"],
        signatures: &[],
        related_formats: &[],
    },
};
