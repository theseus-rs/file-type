use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_6717026: FileFormat = FileFormat {
    id: 6_717_026,
    source_type: SourceType::Wikidata,
    name: "MOI",
    extensions: &["moi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
