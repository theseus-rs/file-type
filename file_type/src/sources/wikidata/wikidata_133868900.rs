use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133868900: FileType = FileType {
    file_format: &FileFormat {
        id: 133_868_900,
        source_type: SourceType::Wikidata,
        name: "STOS Picture Packer medium resolution",
        extensions: &["pp2"],
        media_types: &["image/x-stos-picturepacker-medres"],
        signatures: &[],
        related_formats: &[],
    },
};
