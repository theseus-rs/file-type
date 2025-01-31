use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851171: FileFormat = FileFormat {
    id: 105_851_171,
    puid: "wikidata/105851171",
    name: "Qt Translation source file",
    extensions: &["ts"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
