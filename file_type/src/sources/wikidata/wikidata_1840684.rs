use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1840684: FileType = FileType {
    file_format: &FileFormat {
        id: 1_840_684,
        source_type: SourceType::Wikidata,
        name: "Privacy-enhanced Electronic Mail",
        extensions: &["pem"],
        media_types: &["application/x-pem-file"],
        signatures: &[],
        related_formats: &[],
    },
};
