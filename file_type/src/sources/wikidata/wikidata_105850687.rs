use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850687: FileFormat = FileFormat {
    id: 105_850_687,
    puid: "wikidata/105850687",
    name: "MapInfo Sea Chart",
    extensions: &["kap"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
