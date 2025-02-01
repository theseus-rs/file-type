use crate::format::FileFormat;

pub(crate) const LINGUIST_228: FileFormat = FileFormat {
    id: 228,
    puid: "linguist/228",
    name: "Wikitext",
    extensions: &["mediawiki", "wiki", "wikitext"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
