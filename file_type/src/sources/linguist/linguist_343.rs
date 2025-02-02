use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_343: FileFormat = FileFormat {
    id: 343,
    source_type: SourceType::Linguist,
    name: "Scheme",
    extensions: &["sch", "scm", "sld", "sls", "sps", "ss"],
    media_types: &["text/x-scheme"],
    internal_signatures: &[],
    related_formats: &[],
};
