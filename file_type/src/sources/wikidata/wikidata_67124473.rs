use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67124473: FileFormat = FileFormat {
    id: 67_124_473,
    puid: "wikidata/67124473",
    name: "Print Artist letterhead file format",
    extensions: &["lth"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
