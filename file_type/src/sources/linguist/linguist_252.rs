use crate::format::FileFormat;

pub(crate) const LINGUIST_252: FileFormat = FileFormat {
    id: 252,
    puid: "linguist/252",
    name: "Nix",
    extensions: &["nix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
