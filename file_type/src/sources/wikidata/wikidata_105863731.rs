use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863731: FileFormat = FileFormat {
    id: 105_863_731,
    puid: "wikidata/105863731",
    name: "Mapping Specification Language (ASCII)",
    extensions: &["msl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
