use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857670: FileFormat = FileFormat {
    id: 105_857_670,
    source_type: SourceType::Wikidata,
    name: "Citrix Independent Computer Architecture",
    extensions: &["ica"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
