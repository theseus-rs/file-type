use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_293366493: FileType = FileType {
    file_format: &FileFormat {
        id: 293_366_493,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvsad-bc+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvsad-bc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
