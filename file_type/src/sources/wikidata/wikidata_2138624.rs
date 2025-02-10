use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2138624: FileType = FileType {
    file_format: &FileFormat {
        id: 2_138_624,
        source_type: SourceType::Wikidata,
        name: "registry file",
        extensions: &["reg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
