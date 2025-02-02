use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28532082: FileFormat = FileFormat {
    id: 28_532_082,
    source_type: SourceType::Wikidata,
    name: "CAChe MolStruct",
    extensions: &["cac", "cache"],
    media_types: &["chemical/x-cache"],
    internal_signatures: &[],
    related_formats: &[],
};
