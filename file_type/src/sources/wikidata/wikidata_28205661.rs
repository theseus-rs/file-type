use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205661: FileFormat = FileFormat {
    id: 28_205_661,
    puid: "wikidata/28205661",
    name: "Acorn Sprite",
    extensions: &["acorn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
