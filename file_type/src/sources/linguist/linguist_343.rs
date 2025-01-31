use crate::format::FileFormat;

pub(crate) const LINGUIST_343: FileFormat = FileFormat {
    id: 343,
    puid: "linguist/343",
    name: "Scheme",
    extensions: &["sch", "scm", "sld", "sls", "sps", "ss"],
    media_types: &["text/x-scheme"],
    internal_signatures: &[],
    related_formats: &[],
};
