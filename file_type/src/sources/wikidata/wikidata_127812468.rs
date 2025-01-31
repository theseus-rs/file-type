use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127812468: FileFormat = FileFormat {
    id: 127_812_468,
    puid: "wikidata/127812468",
    name: "Objective-C source code file",
    extensions: &["m"],
    media_types: &["text/x-objective-c"],
    internal_signatures: &[],
    related_formats: &[],
};
