use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48106028: FileFormat = FileFormat {
    id: 48_106_028,
    source_type: SourceType::Wikidata,
    name: "Unisys (Sperry) System Data File",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
