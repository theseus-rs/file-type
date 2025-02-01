use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110238221: FileFormat = FileFormat {
    id: 110_238_221,
    puid: "wikidata/110238221",
    name: "FrameImage",
    extensions: &["fmg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
