use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34736164: FileType = FileType {
    file_format: &FileFormat {
        id: 34_736_164,
        source_type: SourceType::Wikidata,
        name: "Simple Vector Format, version 1",
        extensions: &["svf"],
        media_types: &["image/vnd.svf"],
        signatures: &[],
        related_formats: &[],
    },
};
