use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2872910889: FileType = FileType {
    file_format: &FileFormat {
        id: 2_872_910_889,
        source_type: SourceType::Iana,
        name: "vnd.onepagertatp",
        extensions: &[],
        media_types: &["application/vnd.onepagertatp"],
        signatures: &[],
        related_formats: &[],
    },
};
