use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34284959: FileFormat = FileFormat {
    id: 34_284_959,
    source_type: SourceType::Wikidata,
    name: "Perl 6 script",
    extensions: &["pl6"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
