use crate::format::FileFormat;

pub(crate) const LINGUIST_384: FileFormat = FileFormat {
    id: 384,
    puid: "linguist/384",
    name: "VCL",
    extensions: &["vcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
