use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967184: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_184,
        source_type: SourceType::Wikidata,
        name: "FC-M Packer module",
        extensions: &["fcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
