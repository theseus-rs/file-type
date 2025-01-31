use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_74674437: FileFormat = FileFormat {
    id: 74_674_437,
    puid: "wikidata/74674437",
    name: "Kindle app book info",
    extensions: &["ticr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
