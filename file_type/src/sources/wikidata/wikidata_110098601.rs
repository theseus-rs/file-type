use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110098601: FileFormat = FileFormat {
    id: 110_098_601,
    source_type: SourceType::Wikidata,
    name: "EinScan RGE 3D Range File",
    extensions: &["rge"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
