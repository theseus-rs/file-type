use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3841727505: FileFormat = FileFormat {
    id: 3_841_727_505,
    source_type: SourceType::Iana,
    name: "vnd.sealed.mht",
    extensions: &[],
    media_types: &["application/vnd.sealed.mht"],
    internal_signatures: &[],
    related_formats: &[],
};
