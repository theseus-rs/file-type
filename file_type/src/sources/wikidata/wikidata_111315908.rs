use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111315908: FileFormat = FileFormat {
    id: 111_315_908,
    source_type: SourceType::Wikidata,
    name: "INRS-Telecommunications audio file",
    extensions: &["inrs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
