use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_436568854: FileFormat = FileFormat {
    id: 436_568_854,
    source_type: SourceType::Linguist,
    name: "Protocol Buffer Text Format",
    extensions: &["pbt", "pbtxt", "textproto"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
