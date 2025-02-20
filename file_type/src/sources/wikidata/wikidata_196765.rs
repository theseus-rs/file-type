use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_196765: FileType = FileType {
    file_format: &FileFormat {
        id: 196_765,
        source_type: SourceType::Wikidata,
        name: "revocation list",
        extensions: &["crl"],
        media_types: &["application/pkix-crl"],
        signatures: &[],
        related_formats: &[],
    },
};
