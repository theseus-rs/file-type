use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125823450: FileFormat = FileFormat {
    id: 125_823_450,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Data file",
    extensions: &["hxr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
