use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1587964: FileFormat = FileFormat {
    id: 1_587_964,
    puid: "wikidata/1587964",
    name: "Harwell-Boeing file format",
    extensions: &["rua"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
