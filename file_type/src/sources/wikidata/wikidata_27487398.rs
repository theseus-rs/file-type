use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27487398: FileFormat = FileFormat {
    id: 27_487_398,
    source_type: SourceType::Wikidata,
    name: "Shapefile geocoding index, ODB format",
    extensions: &["mxs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
