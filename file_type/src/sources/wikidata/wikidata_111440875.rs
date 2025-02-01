use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440875: FileFormat = FileFormat {
    id: 111_440_875,
    puid: "wikidata/111440875",
    name: "Tcl Script",
    extensions: &["tcl"],
    media_types: &["text/tcl"],
    internal_signatures: &[],
    related_formats: &[],
};
