use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485673: FileFormat = FileFormat {
    id: 117_485_673,
    puid: "wikidata/117485673",
    name: "Audacity Project File (Early)",
    extensions: &["aup"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
