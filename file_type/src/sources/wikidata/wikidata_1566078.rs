use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1566078: FileFormat = FileFormat {
    id: 1_566_078,
    source_type: SourceType::Wikidata,
    name: "HTML Application",
    extensions: &["hta"],
    media_types: &["application/hta"],
    signatures: &[],
    related_formats: &[],
};
