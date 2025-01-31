use crate::format::FileFormat;

pub(crate) const LINGUIST_299: FileFormat = FileFormat {
    id: 299,
    puid: "linguist/299",
    name: "Puppet",
    extensions: &["pp"],
    media_types: &["text/x-puppet"],
    internal_signatures: &[],
    related_formats: &[],
};
