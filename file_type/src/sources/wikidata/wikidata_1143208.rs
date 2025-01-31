use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1143208: FileFormat = FileFormat {
    id: 1_143_208,
    puid: "wikidata/1143208",
    name: "Cue sheet",
    extensions: &["cue"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
