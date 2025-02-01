use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112944076: FileFormat = FileFormat {
    id: 112_944_076,
    puid: "wikidata/112944076",
    name: "GameExchange2 lights file",
    extensions: &["GLF"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
