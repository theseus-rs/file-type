use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_13039854: FileFormat = FileFormat {
    id: 13_039_854,
    puid: "wikidata/13039854",
    name: "C++ header",
    extensions: &["h", "hh", "hpp", "hxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
