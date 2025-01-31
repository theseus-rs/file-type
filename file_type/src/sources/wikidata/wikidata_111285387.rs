use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111285387: FileFormat = FileFormat {
    id: 111_285_387,
    puid: "wikidata/111285387",
    name: "Sound Tools HCOM format",
    extensions: &["hcom"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
