use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130601735: FileFormat = FileFormat {
    id: 130_601_735,
    puid: "wikidata/130601735",
    name: "R console transcript file",
    extensions: &["rout"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
