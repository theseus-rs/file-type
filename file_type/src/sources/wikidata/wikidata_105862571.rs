use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862571: FileFormat = FileFormat {
    id: 105_862_571,
    puid: "wikidata/105862571",
    name: "Poser Material (V5)",
    extensions: &["mt5"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
