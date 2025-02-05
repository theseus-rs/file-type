use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130466597: FileFormat = FileFormat {
    id: 130_466_597,
    source_type: SourceType::Wikidata,
    name: "Parsing Expression Grammar file format",
    extensions: &["peg"],
    media_types: &["text/x-peg"],
    signatures: &[],
    related_formats: &[],
};
