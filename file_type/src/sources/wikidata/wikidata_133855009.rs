use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133855009: FileType = FileType {
    file_format: &FileFormat {
        id: 133_855_009,
        source_type: SourceType::Wikidata,
        name: "STOS Picture Packer",
        extensions: &["pac"],
        media_types: &["image/x-stos-picturepacker"],
        signatures: &[],
        related_formats: &[],
    },
};
