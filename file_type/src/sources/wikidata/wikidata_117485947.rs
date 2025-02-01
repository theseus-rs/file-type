use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485947: FileFormat = FileFormat {
    id: 117_485_947,
    puid: "wikidata/117485947",
    name: "Audacity Project File 2.x",
    extensions: &["aup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
