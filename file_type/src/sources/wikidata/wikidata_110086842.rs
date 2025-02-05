use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110086842: FileFormat = FileFormat {
    id: 110_086_842,
    source_type: SourceType::Wikidata,
    name: "Agisoft Point Cloud",
    extensions: &["oc3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
