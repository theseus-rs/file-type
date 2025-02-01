use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1312725: FileFormat = FileFormat {
    id: 1_312_725,
    puid: "wikidata/1312725",
    name: "local shared object",
    extensions: &["sol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
