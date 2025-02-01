use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979371: FileFormat = FileFormat {
    id: 27_979_371,
    puid: "wikidata/27979371",
    name: "EBU Timed Text",
    extensions: &["ttml"],
    media_types: &["application/ttml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
