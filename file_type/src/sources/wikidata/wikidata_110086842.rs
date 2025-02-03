use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110086842: FileFormat = FileFormat {
    id: 110_086_842,
    source_type: SourceType::Wikidata,
    name: "Agisoft Point Cloud",
    extensions: &["oc3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
