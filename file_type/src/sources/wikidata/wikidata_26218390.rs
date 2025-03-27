use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26218390: FileType = FileType {
    file_format: &FileFormat {
        id: 26_218_390,
        source_type: SourceType::Wikidata,
        name: "National Imagery Transmission Format, version 1.1",
        extensions: &["nitf", "ntf"],
        media_types: &["application/vnd.nitf"],
        signatures: &[],
        related_formats: &[],
    },
};
