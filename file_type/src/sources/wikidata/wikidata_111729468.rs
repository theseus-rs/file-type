use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111729468: FileFormat = FileFormat {
    id: 111_729_468,
    puid: "wikidata/111729468",
    name: "RegMon File",
    extensions: &["rgd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
