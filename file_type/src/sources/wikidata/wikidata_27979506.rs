use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979506: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_506,
        source_type: SourceType::Wikidata,
        name: "Photoshop Transfer Function",
        extensions: &["atf"],
        media_types: &["application/x-photoshop"],
        signatures: &[],
        related_formats: &[],
    },
};
