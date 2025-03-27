use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26218335: FileType = FileType {
    file_format: &FileFormat {
        id: 26_218_335,
        source_type: SourceType::Wikidata,
        name: "National Imagery Transmission Format, version 2.0",
        extensions: &["nitf", "ntf"],
        media_types: &["application/vnd.nitf"],
        signatures: &[],
        related_formats: &[],
    },
};
