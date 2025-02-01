use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47196445: FileFormat = FileFormat {
    id: 47_196_445,
    puid: "wikidata/47196445",
    name: "AppleWorks Database file format, version 5",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
