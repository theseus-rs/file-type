use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27996251: FileFormat = FileFormat {
    id: 27_996_251,
    puid: "wikidata/27996251",
    name: "InnoDB database file",
    extensions: &["ibd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
