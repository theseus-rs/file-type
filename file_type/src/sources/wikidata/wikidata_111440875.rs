use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111440875: FileFormat = FileFormat {
    id: 111_440_875,
    source_type: SourceType::Wikidata,
    name: "Tcl Script",
    extensions: &["tcl"],
    media_types: &["text/tcl"],
    internal_signatures: &[],
    related_formats: &[],
};
