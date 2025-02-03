use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111356257: FileFormat = FileFormat {
    id: 111_356_257,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif ES 'all' format",
    extensions: &["w7a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
