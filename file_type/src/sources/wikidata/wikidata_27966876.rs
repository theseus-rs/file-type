use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966876: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_876,
        source_type: SourceType::Wikidata,
        name: "2SF",
        extensions: &["2sflib", "mini2sf", "smap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
