use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27978797: FileType = FileType {
    file_format: &FileFormat {
        id: 27_978_797,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 Compressed",
        extensions: &["spc"],
        media_types: &["image/x-spectrum512-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
