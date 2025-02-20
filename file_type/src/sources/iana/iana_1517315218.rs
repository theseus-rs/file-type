use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1517315218: FileType = FileType {
    file_format: &FileFormat {
        id: 1_517_315_218,
        source_type: SourceType::Iana,
        name: "vnd.afpc.modca-mediummap",
        extensions: &[],
        media_types: &["application/vnd.afpc.modca-mediummap"],
        signatures: &[],
        related_formats: &[],
    },
};
