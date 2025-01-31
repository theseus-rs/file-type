use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858120: FileFormat = FileFormat {
    id: 105_858_120,
    puid: "wikidata/105858120",
    name: "KLH10 RAW tape image directory (with rem)",
    extensions: &["tdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
