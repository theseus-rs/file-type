use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206830: FileFormat = FileFormat {
    id: 28_206_830,
    puid: "wikidata/28206830",
    name: "Palette Master",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
