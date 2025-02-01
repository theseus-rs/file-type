use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126960675: FileFormat = FileFormat {
    id: 126_960_675,
    puid: "wikidata/126960675",
    name: "VAPI file",
    extensions: &["vapi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
