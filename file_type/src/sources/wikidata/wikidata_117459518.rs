use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117459518: FileType = FileType {
    file_format: &FileFormat {
        id: 117_459_518,
        source_type: SourceType::Wikidata,
        name: "JWPUB",
        extensions: &["jwpub"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
