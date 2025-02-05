use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_48106028: FileFormat = FileFormat {
    id: 48_106_028,
    source_type: SourceType::Wikidata,
    name: "Unisys (Sperry) System Data File",
    extensions: &["sdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
