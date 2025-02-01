use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28756168: FileFormat = FileFormat {
    id: 28_756_168,
    puid: "wikidata/28756168",
    name: "FWKCS SRT file",
    extensions: &["srt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
