use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1674093862: FileType = FileType {
    file_format: &FileFormat {
        id: 1_674_093_862,
        source_type: SourceType::Iana,
        name: "vnd.Mobius.DAF",
        extensions: &[],
        media_types: &["application/vnd.Mobius.DAF"],
        signatures: &[],
        related_formats: &[],
    },
};
