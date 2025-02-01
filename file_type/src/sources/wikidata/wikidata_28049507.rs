use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049507: FileFormat = FileFormat {
    id: 28_049_507,
    puid: "wikidata/28049507",
    name: "NEOchrome",
    extensions: &["neo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
