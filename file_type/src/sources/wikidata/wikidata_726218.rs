use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_726218: FileFormat = FileFormat {
    id: 726_218,
    puid: "wikidata/726218",
    name: "XML User Interface Language",
    extensions: &["xul"],
    media_types: &["application/vnd.mozilla.xul+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
