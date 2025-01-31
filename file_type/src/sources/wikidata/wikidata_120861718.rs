use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120861718: FileFormat = FileFormat {
    id: 120_861_718,
    puid: "wikidata/120861718",
    name: "MyDVD Project",
    extensions: &["dvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
