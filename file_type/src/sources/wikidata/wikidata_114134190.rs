use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114134190: FileFormat = FileFormat {
    id: 114_134_190,
    source_type: SourceType::Wikidata,
    name: "MOPAC dataset format",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
