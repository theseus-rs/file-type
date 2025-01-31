use crate::format::FileFormat;

pub(crate) const LINGUIST_404: FileFormat = FileFormat {
    id: 404,
    puid: "linguist/404",
    name: "XSLT",
    extensions: &["xsl", "xslt"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
