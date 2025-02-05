use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_5616180: FileFormat = FileFormat {
    id: 5_616_180,
    source_type: SourceType::Iana,
    name: "vnd.collabio.xodocuments.document-template",
    extensions: &[],
    media_types: &["application/vnd.collabio.xodocuments.document-template"],
    signatures: &[],
    related_formats: &[],
};
