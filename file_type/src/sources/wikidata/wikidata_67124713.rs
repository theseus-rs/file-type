use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67124713: FileFormat = FileFormat {
    id: 67_124_713,
    puid: "wikidata/67124713",
    name: "Print Artist postcard file format",
    extensions: &["pc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
