use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853708: FileFormat = FileFormat {
    id: 105_853_708,
    source_type: SourceType::Wikidata,
    name: "Adobe Update Manager data",
    extensions: &["aum"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
