use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17622088: FileFormat = FileFormat {
    id: 17_622_088,
    puid: "wikidata/17622088",
    name: "Speedo",
    extensions: &["spd"],
    media_types: &["application/x-font-speedo"],
    internal_signatures: &[],
    related_formats: &[],
};
