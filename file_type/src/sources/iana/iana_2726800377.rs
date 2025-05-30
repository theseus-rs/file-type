use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2726800377: FileType = FileType {
    file_format: &FileFormat {
        id: 2_726_800_377,
        source_type: SourceType::Iana,
        name: "gff3",
        extensions: &[],
        media_types: &["text/gff3"],
        signatures: &[],
        related_formats: &[],
    },
};
