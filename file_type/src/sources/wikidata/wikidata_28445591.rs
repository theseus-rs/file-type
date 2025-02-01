use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445591: FileFormat = FileFormat {
    id: 28_445_591,
    puid: "wikidata/28445591",
    name: "AMOS BASIC tokenized file",
    extensions: &["amos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
