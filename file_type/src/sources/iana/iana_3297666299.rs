use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3297666299: FileType = FileType {
    file_format: &FileFormat {
        id: 3_297_666_299,
        source_type: SourceType::Iana,
        name: "vnd.afpc.modca-formdef",
        extensions: &[],
        media_types: &["application/vnd.afpc.modca-formdef"],
        signatures: &[],
        related_formats: &[],
    },
};
