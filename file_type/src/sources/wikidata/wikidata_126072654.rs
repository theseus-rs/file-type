use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126072654: FileFormat = FileFormat {
    id: 126_072_654,
    puid: "wikidata/126072654",
    name: "WinFax Sent / Received Document file",
    extensions: &["fxr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
