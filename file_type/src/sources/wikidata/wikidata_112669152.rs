use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112669152: FileFormat = FileFormat {
    id: 112_669_152,
    puid: "wikidata/112669152",
    name: "Lightscape Parameter",
    extensions: &["df"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
