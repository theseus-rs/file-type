use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66439259: FileFormat = FileFormat {
    id: 66_439_259,
    source_type: SourceType::Wikidata,
    name: "WordPerfect Merge Data file format",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
