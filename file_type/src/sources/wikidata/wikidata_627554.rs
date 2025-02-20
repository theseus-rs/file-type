use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_627554: FileType = FileType {
    file_format: &FileFormat {
        id: 627_554,
        source_type: SourceType::Wikidata,
        name: "certificate signing request",
        extensions: &["csr", "p10"],
        media_types: &["application/pkcs10"],
        signatures: &[],
        related_formats: &[],
    },
};
