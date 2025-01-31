use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50223921: FileFormat = FileFormat {
    id: 50_223_921,
    puid: "wikidata/50223921",
    name: "Adobe Air",
    extensions: &["air"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
