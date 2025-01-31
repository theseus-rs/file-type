use crate::format::FileFormat;

pub(crate) const LINGUIST_123: FileFormat = FileFormat {
    id: 123,
    puid: "linguist/123",
    name: "GDScript",
    extensions: &["gd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
