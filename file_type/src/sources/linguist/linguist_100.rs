use crate::format::FileFormat;

pub(crate) const LINGUIST_100: FileFormat = FileFormat {
    id: 100,
    puid: "linguist/100",
    name: "Elixir",
    extensions: &["ex", "exs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
