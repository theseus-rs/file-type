use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118218029: FileFormat = FileFormat {
    id: 118_218_029,
    puid: "wikidata/118218029",
    name: "FotoFinish Layout",
    extensions: &["fdd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
