use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1754328778: FileFormat = FileFormat {
    id: 1_754_328_778,
    source_type: SourceType::Iana,
    name: "vnd.japannet-jpnstore-wakeup",
    extensions: &[],
    media_types: &["application/vnd.japannet-jpnstore-wakeup"],
    internal_signatures: &[],
    related_formats: &[],
};
