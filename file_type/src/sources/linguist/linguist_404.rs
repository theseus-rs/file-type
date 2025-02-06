use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_404: FileFormat = FileFormat {
    id: 404,
    source_type: SourceType::Linguist,
    name: "XSLT",
    extensions: &["xsl", "xslt"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
