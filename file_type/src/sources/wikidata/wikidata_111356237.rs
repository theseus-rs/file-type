use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111356237: FileFormat = FileFormat {
    id: 111_356_237,
    source_type: SourceType::Wikidata,
    name: "Yamaha Motif (6/7/8) sample data file",
    extensions: &["w3v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
