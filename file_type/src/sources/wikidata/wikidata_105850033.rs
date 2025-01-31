use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850033: FileFormat = FileFormat {
    id: 105_850_033,
    puid: "wikidata/105850033",
    name: "Xilinx Core Generator System Project (with rem)",
    extensions: &["cgp"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
