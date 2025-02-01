use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207212: FileFormat = FileFormat {
    id: 28_207_212,
    puid: "wikidata/28207212",
    name: "Quantel VPB image",
    extensions: &["vpb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
