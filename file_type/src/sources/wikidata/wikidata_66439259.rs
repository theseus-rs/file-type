use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66439259: FileFormat = FileFormat {
    id: 66_439_259,
    source_type: SourceType::Wikidata,
    name: "WordPerfect Merge Data file format",
    extensions: &["dat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
