use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_25822322: FileFormat = FileFormat {
    id: 25_822_322,
    puid: "wikidata/25822322",
    name: "Protocolbuffer Binary Format",
    extensions: &["osm.pbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
