use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60558525: FileFormat = FileFormat {
    id: 60_558_525,
    puid: "wikidata/60558525",
    name: "ClarisWorks Drawing file format, version 2",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
