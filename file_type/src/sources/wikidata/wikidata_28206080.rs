use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206080: FileFormat = FileFormat {
    id: 28_206_080,
    puid: "wikidata/28206080",
    name: "PI6",
    extensions: &["PI6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
