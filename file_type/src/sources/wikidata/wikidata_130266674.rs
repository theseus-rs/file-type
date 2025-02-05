use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130266674: FileFormat = FileFormat {
    id: 130_266_674,
    source_type: SourceType::Wikidata,
    name: "Luau source code file",
    extensions: &["luau"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
