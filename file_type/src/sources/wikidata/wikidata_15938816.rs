use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_15938816: FileFormat = FileFormat {
    id: 15_938_816,
    puid: "wikidata/15938816",
    name: "MATLAB M-File",
    extensions: &["m"],
    media_types: &["text/matlab"],
    internal_signatures: &[],
    related_formats: &[],
};
