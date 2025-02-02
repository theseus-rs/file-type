use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857670: FileFormat = FileFormat {
    id: 105_857_670,
    source_type: SourceType::Wikidata,
    name: "Citrix Independent Computer Architecture",
    extensions: &["ica"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
