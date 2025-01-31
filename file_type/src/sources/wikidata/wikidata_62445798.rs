use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62445798: FileFormat = FileFormat {
    id: 62_445_798,
    puid: "wikidata/62445798",
    name: "OWL XML Serialization",
    extensions: &["owx"],
    media_types: &["application/owl+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
