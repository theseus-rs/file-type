use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1886613558: FileFormat = FileFormat {
    id: 1_886_613_558,
    source_type: SourceType::Iana,
    name: "vnd.ubisoft.webplayer",
    extensions: &[],
    media_types: &["application/vnd.ubisoft.webplayer"],
    internal_signatures: &[],
    related_formats: &[],
};
