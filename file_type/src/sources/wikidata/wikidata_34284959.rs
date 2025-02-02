use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34284959: FileFormat = FileFormat {
    id: 34_284_959,
    source_type: SourceType::Wikidata,
    name: "Perl 6 script",
    extensions: &["pl6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
