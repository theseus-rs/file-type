use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125822754: FileFormat = FileFormat {
    id: 125_822_754,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Index",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
