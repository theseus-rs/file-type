use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130536808: FileFormat = FileFormat {
    id: 130_536_808,
    puid: "wikidata/130536808",
    name: "PRQL source code file",
    extensions: &["prql", "prql"],
    media_types: &["application/prql", "application/x-prql"],
    internal_signatures: &[],
    related_formats: &[],
};
