use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130466597: FileFormat = FileFormat {
    id: 130_466_597,
    source_type: SourceType::Wikidata,
    name: "Parsing Expression Grammar file format",
    extensions: &["peg"],
    media_types: &["text/x-peg"],
    internal_signatures: &[],
    related_formats: &[],
};
