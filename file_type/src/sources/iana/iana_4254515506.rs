use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4254515506: FileType = FileType {
    file_format: &FileFormat {
        id: 4_254_515_506,
        source_type: SourceType::Iana,
        name: "vnd.oftn.l10n+json",
        extensions: &[],
        media_types: &["application/vnd.oftn.l10n+json"],
        signatures: &[],
        related_formats: &[],
    },
};
