use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25822458: FileFormat = FileFormat {
    id: 25_822_458,
    source_type: SourceType::Wikidata,
    name: "OsmChange",
    extensions: &["osc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
