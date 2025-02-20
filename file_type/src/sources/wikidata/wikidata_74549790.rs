use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_74549790: FileType = FileType {
    file_format: &FileFormat {
        id: 74_549_790,
        source_type: SourceType::Wikidata,
        name: "Expert Witness compression Format SMART disk image",
        extensions: &["s01"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
