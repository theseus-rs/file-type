use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2452848747: FileFormat = FileFormat {
    id: 2_452_848_747,
    source_type: SourceType::Iana,
    name: "vnd.iptvforum.2dparityfec-2005",
    extensions: &[],
    media_types: &["video/vnd.iptvforum.2dparityfec-2005"],
    internal_signatures: &[],
    related_formats: &[],
};
