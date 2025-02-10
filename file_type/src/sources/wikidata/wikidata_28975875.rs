use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975875: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_875,
        source_type: SourceType::Wikidata,
        name: "OOGL TLIST Projective file",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
