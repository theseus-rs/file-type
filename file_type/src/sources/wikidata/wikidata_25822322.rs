use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_25822322: FileFormat = FileFormat {
    id: 25_822_322,
    source_type: SourceType::Wikidata,
    name: "Protocolbuffer Binary Format",
    extensions: &["osm.pbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
