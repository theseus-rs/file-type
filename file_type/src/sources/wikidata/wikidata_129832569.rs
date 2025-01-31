use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129832569: FileFormat = FileFormat {
    id: 129_832_569,
    puid: "wikidata/129832569",
    name: "Isabelle file format",
    extensions: &["thy"],
    media_types: &["text/x-isabelle"],
    internal_signatures: &[],
    related_formats: &[],
};
