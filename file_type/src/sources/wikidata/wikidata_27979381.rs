use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979381: FileFormat = FileFormat {
    id: 27_979_381,
    source_type: SourceType::Wikidata,
    name: "Blu-ray Clip info",
    extensions: &["clp", "clpi", "cpi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
