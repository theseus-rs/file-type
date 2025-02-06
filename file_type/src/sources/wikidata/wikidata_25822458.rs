use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25822458: FileFormat = FileFormat {
    id: 25_822_458,
    source_type: SourceType::Wikidata,
    name: "OsmChange",
    extensions: &["osc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
