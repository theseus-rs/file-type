use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75597003: FileFormat = FileFormat {
    id: 75_597_003,
    source_type: SourceType::Wikidata,
    name: "GeoGebra format, version 1.0",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    signatures: &[],
    related_formats: &[],
};
