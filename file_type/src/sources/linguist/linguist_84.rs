use crate::format::FileFormat;

pub(crate) const LINGUIST_84: FileFormat = FileFormat {
    id: 84,
    puid: "linguist/84",
    name: "DNS Zone",
    extensions: &["arpa", "zone"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
