use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111729224: FileType = FileType {
    file_format: &FileFormat {
        id: 111_729_224,
        source_type: SourceType::Wikidata,
        name: "Document Manager file",
        extensions: &["ddm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
