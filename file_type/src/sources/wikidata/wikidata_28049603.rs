use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28049603: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_603,
        source_type: SourceType::Wikidata,
        name: "Tiny Stuff, medium resolution",
        extensions: &["tn2", "tny"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
