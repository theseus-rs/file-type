use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3236082836: FileFormat = FileFormat {
    id: 3_236_082_836,
    source_type: SourceType::Iana,
    name: "vnd.radisys.msml-dialog-group+xml",
    extensions: &[],
    media_types: &["application/vnd.radisys.msml-dialog-group+xml"],
    signatures: &[],
    related_formats: &[],
};
