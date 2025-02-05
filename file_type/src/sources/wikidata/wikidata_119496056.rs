use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119496056: FileFormat = FileFormat {
    id: 119_496_056,
    source_type: SourceType::Wikidata,
    name: "IBM IOCA Raw",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
