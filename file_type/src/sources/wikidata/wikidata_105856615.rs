use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856615: FileFormat = FileFormat {
    id: 105_856_615,
    source_type: SourceType::Wikidata,
    name: "WiX Project",
    extensions: &["wixproj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
