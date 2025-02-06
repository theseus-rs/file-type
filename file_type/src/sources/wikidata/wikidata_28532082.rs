use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28532082: FileFormat = FileFormat {
    id: 28_532_082,
    source_type: SourceType::Wikidata,
    name: "CAChe MolStruct",
    extensions: &["cac", "cache"],
    media_types: &["chemical/x-cache"],
    signatures: &[],
    related_formats: &[],
};
