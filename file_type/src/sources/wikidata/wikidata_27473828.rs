use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473828: FileFormat = FileFormat {
    id: 27_473_828,
    puid: "wikidata/27473828",
    name: "BIL/BIP/BSQ Color File",
    extensions: &["clr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
