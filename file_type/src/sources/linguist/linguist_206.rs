use crate::format::FileFormat;

pub(crate) const LINGUIST_206: FileFormat = FileFormat {
    id: 206,
    puid: "linguist/206",
    name: "Literate CoffeeScript",
    extensions: &["coffee.md", "litcoffee"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
