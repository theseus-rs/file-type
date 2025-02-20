use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27225795: FileType = FileType {
    file_format: &FileFormat {
        id: 27_225_795,
        source_type: SourceType::Wikidata,
        name: "OpenDocument Graphics, version 1.0",
        extensions: &["odg"],
        media_types: &["application/vnd.oasis.opendocument.graphics"],
        signatures: &[],
        related_formats: &[],
    },
};
