use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959836: FileFormat = FileFormat {
    id: 27_959_836,
    puid: "wikidata/27959836",
    name: "Raw FL Studio Project",
    extensions: &["flp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
