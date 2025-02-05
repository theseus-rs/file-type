use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125823450: FileFormat = FileFormat {
    id: 125_823_450,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Data file",
    extensions: &["hxr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
