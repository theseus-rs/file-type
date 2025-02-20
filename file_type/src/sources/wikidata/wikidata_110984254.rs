use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110984254: FileType = FileType {
    file_format: &FileFormat {
        id: 110_984_254,
        source_type: SourceType::Wikidata,
        name: "Corel VideoStudio Project File",
        extensions: &["vsp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
