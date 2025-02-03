use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3770995111: FileFormat = FileFormat {
    id: 3_770_995_111,
    source_type: SourceType::Iana,
    name: "vnd.ecowin.fileupdate",
    extensions: &[],
    media_types: &["application/vnd.ecowin.fileupdate"],
    internal_signatures: &[],
    related_formats: &[],
};
