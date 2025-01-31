use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55753012: FileFormat = FileFormat {
    id: 55_753_012,
    puid: "wikidata/55753012",
    name: "Microsoft xWMA file format",
    extensions: &["xwma"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
