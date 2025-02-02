use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856615: FileFormat = FileFormat {
    id: 105_856_615,
    source_type: SourceType::Wikidata,
    name: "WiX Project",
    extensions: &["wixproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
