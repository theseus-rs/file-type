use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4102308401: FileType = FileType {
    file_format: &FileFormat {
        id: 4_102_308_401,
        source_type: SourceType::Iana,
        name: "alto-endpointcostparams+json",
        extensions: &[],
        media_types: &["application/alto-endpointcostparams+json"],
        signatures: &[],
        related_formats: &[],
    },
};
