use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1798121: FileFormat = FileFormat {
    id: 1_798_121,
    source_type: SourceType::Wikidata,
    name: "Microsoft Library",
    extensions: &["lib"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
