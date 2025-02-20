use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111272295: FileType = FileType {
    file_format: &FileFormat {
        id: 111_272_295,
        source_type: SourceType::Wikidata,
        name: "Ensoniq disk image",
        extensions: &["edx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
