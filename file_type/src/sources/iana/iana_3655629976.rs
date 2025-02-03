use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3655629976: FileFormat = FileFormat {
    id: 3_655_629_976,
    source_type: SourceType::Iana,
    name: "vnd.triscape.mxs",
    extensions: &[],
    media_types: &["application/vnd.triscape.mxs"],
    internal_signatures: &[],
    related_formats: &[],
};
