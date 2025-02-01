use crate::format::FileFormat;

pub(crate) const LINGUIST_145: FileFormat = FileFormat {
    id: 145,
    puid: "linguist/145",
    name: "HLSL",
    extensions: &["cginc", "fx", "fxh", "hlsl", "hlsli"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
