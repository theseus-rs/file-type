use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66142150: FileFormat = FileFormat {
    id: 66_142_150,
    source_type: SourceType::Wikidata,
    name: "ADE file format",
    extensions: &["ade"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
