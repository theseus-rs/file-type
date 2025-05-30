use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2842089512: FileType = FileType {
    file_format: &FileFormat {
        id: 2_842_089_512,
        source_type: SourceType::Iana,
        name: "vnd.nintendo.nitro.rom",
        extensions: &[],
        media_types: &["application/vnd.nintendo.nitro.rom"],
        signatures: &[],
        related_formats: &[],
    },
};
