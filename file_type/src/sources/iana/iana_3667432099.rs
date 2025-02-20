use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3667432099: FileType = FileType {
    file_format: &FileFormat {
        id: 3_667_432_099,
        source_type: SourceType::Iana,
        name: "vnd.afpc.modca-cmtable",
        extensions: &[],
        media_types: &["application/vnd.afpc.modca-cmtable"],
        signatures: &[],
        related_formats: &[],
    },
};
