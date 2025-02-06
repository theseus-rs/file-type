use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_23014810: FileFormat = FileFormat {
    id: 23_014_810,
    source_type: SourceType::Wikidata,
    name: "chr",
    extensions: &["chr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
