use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111520019: FileFormat = FileFormat {
    id: 111_520_019,
    puid: "wikidata/111520019",
    name: "R program file",
    extensions: &["r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
