use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111356350: FileFormat = FileFormat {
    id: 111_356_350,
    source_type: SourceType::Wikidata,
    name: "Turtle Beach WaveFront program format",
    extensions: &["wfp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
