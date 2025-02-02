use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47012439: FileFormat = FileFormat {
    id: 47_012_439,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Document file format",
    extensions: &["bps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
