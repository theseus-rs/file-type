use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130536808: FileFormat = FileFormat {
    id: 130_536_808,
    source_type: SourceType::Wikidata,
    name: "PRQL source code file",
    extensions: &["prql"],
    media_types: &["application/prql", "application/x-prql"],
    signatures: &[],
    related_formats: &[],
};
