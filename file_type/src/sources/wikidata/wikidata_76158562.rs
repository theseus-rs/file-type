use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_76158562: FileType = FileType {
    file_format: &FileFormat {
        id: 76_158_562,
        source_type: SourceType::Wikidata,
        name: "VisKit 3d model",
        extensions: &["vk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
