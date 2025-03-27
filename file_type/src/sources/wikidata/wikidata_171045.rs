use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_171045: FileType = FileType {
    file_format: &FileFormat {
        id: 171_045,
        source_type: SourceType::Wikidata,
        name: "Open Virtualization Format",
        extensions: &[],
        media_types: &["application/ovf"],
        signatures: &[],
        related_formats: &[],
    },
};
