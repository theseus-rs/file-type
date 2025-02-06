use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_25822322: FileFormat = FileFormat {
    id: 25_822_322,
    source_type: SourceType::Wikidata,
    name: "Protocolbuffer Binary Format",
    extensions: &["osm.pbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
