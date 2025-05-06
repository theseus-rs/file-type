use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133855124: FileType = FileType {
    file_format: &FileFormat {
        id: 133_855_124,
        source_type: SourceType::Wikidata,
        name: "STOS Picture Packer low resolution",
        extensions: &["pp1"],
        media_types: &["image/x-stos-picturepacker-lowres"],
        signatures: &[],
        related_formats: &[],
    },
};
