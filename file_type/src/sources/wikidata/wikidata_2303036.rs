use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2303036: FileType = FileType {
    file_format: &FileFormat {
        id: 2_303_036,
        source_type: SourceType::Wikidata,
        name: "WWF, unprintable PDF",
        extensions: &["wwf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
