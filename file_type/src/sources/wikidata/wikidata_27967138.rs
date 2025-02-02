use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967138: FileFormat = FileFormat {
    id: 27_967_138,
    source_type: SourceType::Wikidata,
    name: "DigiBooster v1.x module",
    extensions: &["digi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
