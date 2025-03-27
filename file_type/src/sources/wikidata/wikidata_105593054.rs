use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105593054: FileType = FileType {
    file_format: &FileFormat {
        id: 105_593_054,
        source_type: SourceType::Wikidata,
        name: "Portable Float Map, grayscale variant",
        extensions: &["pfm", "pnm"],
        media_types: &["image/x-portable-anymap", "image/x-portable-floatmap"],
        signatures: &[],
        related_formats: &[],
    },
};
