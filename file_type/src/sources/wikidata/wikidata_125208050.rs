use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125208050: FileFormat = FileFormat {
    id: 125_208_050,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project XML",
    extensions: &["mspxml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
