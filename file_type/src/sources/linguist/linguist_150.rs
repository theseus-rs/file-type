use crate::format::FileFormat;

pub(crate) const LINGUIST_150: FileFormat = FileFormat {
    id: 150,
    puid: "linguist/150",
    name: "HTML+ERB",
    extensions: &["erb", "erb.deface", "rhtml"],
    media_types: &["application/x-erb"],
    internal_signatures: &[],
    related_formats: &[],
};
