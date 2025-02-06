use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_39170567: FileFormat = FileFormat {
    id: 39_170_567,
    source_type: SourceType::Wikidata,
    name: "GeoGebra tool",
    extensions: &["ggt"],
    media_types: &["application/vnd.geogebra.tool"],
    signatures: &[],
    related_formats: &[],
};
