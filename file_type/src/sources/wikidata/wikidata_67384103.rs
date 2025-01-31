use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67384103: FileFormat = FileFormat {
    id: 67_384_103,
    puid: "wikidata/67384103",
    name: "ArtMoney Table File",
    extensions: &["amt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
