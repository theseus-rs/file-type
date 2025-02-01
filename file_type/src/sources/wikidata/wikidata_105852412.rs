use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852412: FileFormat = FileFormat {
    id: 105_852_412,
    puid: "wikidata/105852412",
    name: "SExtractor configuration",
    extensions: &["sex"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
