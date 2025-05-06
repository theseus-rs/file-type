use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133855049: FileType = FileType {
    file_format: &FileFormat {
        id: 133_855_049,
        source_type: SourceType::Wikidata,
        name: "STOS Picture Packer high resolution",
        extensions: &["pp3"],
        media_types: &["image/x-stos-picturepacker-hires"],
        signatures: &[],
        related_formats: &[],
    },
};
