use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109239421: FileFormat = FileFormat {
    id: 109_239_421,
    source_type: SourceType::Wikidata,
    name: "HMAPPS Document",
    extensions: &["mv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
