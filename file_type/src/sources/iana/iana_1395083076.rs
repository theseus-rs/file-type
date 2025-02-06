use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1395083076: FileFormat = FileFormat {
    id: 1_395_083_076,
    source_type: SourceType::Iana,
    name: "vnd.collabio.xodocuments.spreadsheet-template",
    extensions: &[],
    media_types: &["application/vnd.collabio.xodocuments.spreadsheet-template"],
    signatures: &[],
    related_formats: &[],
};
