use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_15491224: FileFormat = FileFormat {
    id: 15_491_224,
    source_type: SourceType::Iana,
    name: "vnd.gmx - DEPRECATED",
    extensions: &[],
    media_types: &["application/vnd.gmx"],
    internal_signatures: &[],
    related_formats: &[],
};
