use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3268673863: FileType = FileType {
    file_format: &FileFormat {
        id: 3_268_673_863,
        source_type: SourceType::Iana,
        name: "vnd.afpc.afplinedata-pagedef",
        extensions: &[],
        media_types: &["application/vnd.afpc.afplinedata-pagedef"],
        signatures: &[],
        related_formats: &[],
    },
};
