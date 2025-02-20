use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205541: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_541,
        source_type: SourceType::Wikidata,
        name: "NeoDesk icon",
        extensions: &["nic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
