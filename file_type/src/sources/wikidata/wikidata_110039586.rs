use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110039586: FileFormat = FileFormat {
    id: 110_039_586,
    source_type: SourceType::Wikidata,
    name: "Micrografx In-A-Vision Drawing",
    extensions: &["pic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
