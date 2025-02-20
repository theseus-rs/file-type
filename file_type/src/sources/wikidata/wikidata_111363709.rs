use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111363709: FileType = FileType {
    file_format: &FileFormat {
        id: 111_363_709,
        source_type: SourceType::Wikidata,
        name: "Yamaha Motif XF 'voices' format",
        extensions: &["x3v"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
