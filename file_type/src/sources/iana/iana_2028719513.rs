use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2028719513: FileFormat = FileFormat {
    id: 2_028_719_513,
    source_type: SourceType::Iana,
    name: "vnd.sealed.tiff",
    extensions: &[],
    media_types: &["application/vnd.sealed.tiff"],
    internal_signatures: &[],
    related_formats: &[],
};
