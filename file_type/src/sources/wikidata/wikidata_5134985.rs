use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_5134985: FileFormat = FileFormat {
    id: 5_134_985,
    puid: "wikidata/5134985",
    name: "CloneCD Control File",
    extensions: &["ccd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
