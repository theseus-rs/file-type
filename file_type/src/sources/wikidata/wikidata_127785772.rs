use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127785772: FileFormat = FileFormat {
    id: 127_785_772,
    puid: "wikidata/127785772",
    name: "MEL script",
    extensions: &["mel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
