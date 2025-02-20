use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_20191913: FileType = FileType {
    file_format: &FileFormat {
        id: 20_191_913,
        source_type: SourceType::Wikidata,
        name: "Apple Help File Format",
        extensions: &["lproj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
