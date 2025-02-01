use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777687: FileFormat = FileFormat {
    id: 28_777_687,
    puid: "wikidata/28777687",
    name: "Mono",
    extensions: &["mono"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
