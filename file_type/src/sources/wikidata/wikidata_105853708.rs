use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853708: FileFormat = FileFormat {
    id: 105_853_708,
    source_type: SourceType::Wikidata,
    name: "Adobe Update Manager data",
    extensions: &["aum"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
