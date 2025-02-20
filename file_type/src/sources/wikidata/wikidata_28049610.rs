use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28049610: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_610,
        source_type: SourceType::Wikidata,
        name: "Tiny Stuff, high resolution",
        extensions: &["tn3", "tny"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
