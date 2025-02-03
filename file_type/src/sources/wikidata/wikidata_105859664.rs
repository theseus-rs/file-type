use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859664: FileFormat = FileFormat {
    id: 105_859_664,
    source_type: SourceType::Wikidata,
    name: "Verilog source code (with pre)",
    extensions: &["v"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
