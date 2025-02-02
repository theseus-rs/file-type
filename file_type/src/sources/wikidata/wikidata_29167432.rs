use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29167432: FileFormat = FileFormat {
    id: 29_167_432,
    source_type: SourceType::Wikidata,
    name: "NuFX",
    extensions: &["bxy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
