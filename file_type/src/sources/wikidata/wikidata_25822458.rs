use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25822458: FileFormat = FileFormat {
    id: 25_822_458,
    puid: "wikidata/25822458",
    name: "OsmChange",
    extensions: &["osc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
