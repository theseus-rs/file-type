use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118669892: FileFormat = FileFormat {
    id: 118_669_892,
    source_type: SourceType::Wikidata,
    name: "Layer Link File",
    extensions: &["clk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
