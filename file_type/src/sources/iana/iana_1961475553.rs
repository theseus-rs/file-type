use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1961475553: FileType = FileType {
    file_format: &FileFormat {
        id: 1_961_475_553,
        source_type: SourceType::Iana,
        name: "vnd.Mobius.DIS",
        extensions: &[],
        media_types: &["application/vnd.Mobius.DIS"],
        signatures: &[],
        related_formats: &[],
    },
};
