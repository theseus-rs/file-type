use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100235620: FileFormat = FileFormat {
    id: 100_235_620,
    puid: "wikidata/100235620",
    name: "FARO WorkSpace File",
    extensions: &["fws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
