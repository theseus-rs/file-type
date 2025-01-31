use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111182155: FileFormat = FileFormat {
    id: 111_182_155,
    puid: "wikidata/111182155",
    name: "Dreamweaver Library Item",
    extensions: &["lbi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
