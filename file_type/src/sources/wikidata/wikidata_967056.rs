use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_967056: FileFormat = FileFormat {
    id: 967_056,
    puid: "wikidata/967056",
    name: "SoundFont",
    extensions: &["sf2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
