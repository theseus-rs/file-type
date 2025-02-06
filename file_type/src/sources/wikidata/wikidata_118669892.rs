use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118669892: FileFormat = FileFormat {
    id: 118_669_892,
    source_type: SourceType::Wikidata,
    name: "Layer Link File",
    extensions: &["clk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
