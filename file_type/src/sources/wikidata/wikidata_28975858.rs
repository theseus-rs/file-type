use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975858: FileFormat = FileFormat {
    id: 28_975_858,
    puid: "wikidata/28975858",
    name: "OOGL QUAD file",
    extensions: &["quad"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
