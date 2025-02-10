use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27978797: FileType = FileType {
    file_format: &FileFormat {
        id: 27_978_797,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 Compressed",
        extensions: &["spc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
