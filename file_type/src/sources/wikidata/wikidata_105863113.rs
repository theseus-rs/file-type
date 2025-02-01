use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863113: FileFormat = FileFormat {
    id: 105_863_113,
    puid: "wikidata/105863113",
    name: "mzXML",
    extensions: &["mzXML"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
