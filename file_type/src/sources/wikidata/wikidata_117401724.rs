use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117401724: FileFormat = FileFormat {
    id: 117_401_724,
    source_type: SourceType::Wikidata,
    name: "NBT",
    extensions: &["dat", "nbt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
