use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111356337: FileFormat = FileFormat {
    id: 111_356_337,
    source_type: SourceType::Wikidata,
    name: "Turtle Beach WaveFront drum set format",
    extensions: &["wfd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
