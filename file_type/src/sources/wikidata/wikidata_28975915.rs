use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975915: FileFormat = FileFormat {
    id: 28_975_915,
    puid: "wikidata/28975915",
    name: "ZPR",
    extensions: &["zpr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
