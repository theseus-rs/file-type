use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111356337: FileFormat = FileFormat {
    id: 111_356_337,
    source_type: SourceType::Wikidata,
    name: "Turtle Beach WaveFront drum set format",
    extensions: &["wfd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
