use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4786175: FileType = FileType {
    file_format: &FileFormat {
        id: 4_786_175,
        source_type: SourceType::Wikidata,
        name: "ArchiCAD library part",
        extensions: &["gsm"],
        media_types: &["model/vnd.gs-gdl"],
        signatures: &[],
        related_formats: &[],
    },
};
