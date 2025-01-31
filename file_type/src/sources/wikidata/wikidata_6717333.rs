use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6717333: FileFormat = FileFormat {
    id: 6_717_333,
    puid: "wikidata/6717333",
    name: "Mathematical Programming System",
    extensions: &["mps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
