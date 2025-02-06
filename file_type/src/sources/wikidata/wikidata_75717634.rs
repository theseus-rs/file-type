use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75717634: FileFormat = FileFormat {
    id: 75_717_634,
    source_type: SourceType::Wikidata,
    name: "Corel Photo Paint User Defined Filter",
    extensions: &["usr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
