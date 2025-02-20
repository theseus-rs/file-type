use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979370: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_370,
        source_type: SourceType::Wikidata,
        name: "EBU STL",
        extensions: &["stl"],
        media_types: &["application/x-ebu-stl"],
        signatures: &[],
        related_formats: &[],
    },
};
