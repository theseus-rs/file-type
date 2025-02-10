use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_115736167: FileType = FileType {
    file_format: &FileFormat {
        id: 115_736_167,
        source_type: SourceType::Iana,
        name: "vnd.wolfram.mathematica.package",
        extensions: &[],
        media_types: &["application/vnd.wolfram.mathematica.package"],
        signatures: &[],
        related_formats: &[],
    },
};
