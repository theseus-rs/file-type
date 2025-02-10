use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_672006244: FileType = FileType {
    file_format: &FileFormat {
        id: 672_006_244,
        source_type: SourceType::Iana,
        name: "prs.prop.logic",
        extensions: &[],
        media_types: &["text/prs.prop.logic"],
        signatures: &[],
        related_formats: &[],
    },
};
