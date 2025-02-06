use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855949: FileFormat = FileFormat {
    id: 105_855_949,
    source_type: SourceType::Wikidata,
    name: "DMIS input data",
    extensions: &["dmi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
