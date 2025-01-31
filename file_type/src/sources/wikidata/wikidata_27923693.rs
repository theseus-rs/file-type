use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27923693: FileFormat = FileFormat {
    id: 27_923_693,
    puid: "wikidata/27923693",
    name: "DTED Cells Directory File",
    extensions: &["dir"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
