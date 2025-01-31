use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120635955: FileFormat = FileFormat {
    id: 120_635_955,
    puid: "wikidata/120635955",
    name: "Microsoft Data Access Page",
    extensions: &["htm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
