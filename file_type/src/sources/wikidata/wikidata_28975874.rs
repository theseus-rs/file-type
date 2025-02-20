use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975874: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_874,
        source_type: SourceType::Wikidata,
        name: "OOGL TLIST Group file",
        extensions: &["grp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
