use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47520795: FileType = FileType {
    file_format: &FileFormat {
        id: 47_520_795,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version 11",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
