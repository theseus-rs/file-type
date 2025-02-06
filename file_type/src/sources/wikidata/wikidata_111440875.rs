use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111440875: FileFormat = FileFormat {
    id: 111_440_875,
    source_type: SourceType::Wikidata,
    name: "Tcl Script",
    extensions: &["tcl"],
    media_types: &["text/tcl"],
    signatures: &[],
    related_formats: &[],
};
