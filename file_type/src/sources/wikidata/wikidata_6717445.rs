use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6717445: FileFormat = FileFormat {
    id: 6_717_445,
    source_type: SourceType::Wikidata,
    name: "MRC",
    extensions: &["mrc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
