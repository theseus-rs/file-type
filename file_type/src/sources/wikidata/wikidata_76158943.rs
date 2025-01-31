use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_76158943: FileFormat = FileFormat {
    id: 76_158_943,
    puid: "wikidata/76158943",
    name: "MegaPaint VPO",
    extensions: &["vpo"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
