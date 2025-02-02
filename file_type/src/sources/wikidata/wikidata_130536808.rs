use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130536808: FileFormat = FileFormat {
    id: 130_536_808,
    source_type: SourceType::Wikidata,
    name: "PRQL source code file",
    extensions: &["prql"],
    media_types: &["application/prql", "application/x-prql"],
    internal_signatures: &[],
    related_formats: &[],
};
