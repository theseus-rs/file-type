use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854907: FileFormat = FileFormat {
    id: 105_854_907,
    puid: "wikidata/105854907",
    name: "Artificial Intelligence Markup Language",
    extensions: &["aiml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
