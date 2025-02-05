use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110098601: FileFormat = FileFormat {
    id: 110_098_601,
    source_type: SourceType::Wikidata,
    name: "EinScan RGE 3D Range File",
    extensions: &["rge"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
