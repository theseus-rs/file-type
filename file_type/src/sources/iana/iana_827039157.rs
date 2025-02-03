use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_827039157: FileFormat = FileFormat {
    id: 827_039_157,
    source_type: SourceType::Iana,
    name: "vnd.Mobius.TXF",
    extensions: &[],
    media_types: &["application/vnd.Mobius.TXF"],
    internal_signatures: &[],
    related_formats: &[],
};
