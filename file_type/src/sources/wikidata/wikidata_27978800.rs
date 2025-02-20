use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27978800: FileType = FileType {
    file_format: &FileFormat {
        id: 27_978_800,
        source_type: SourceType::Wikidata,
        name: "Spectrum 512 Smooshed",
        extensions: &["sps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
