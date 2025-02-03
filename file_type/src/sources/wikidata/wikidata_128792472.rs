use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_128792472: FileFormat = FileFormat {
    id: 128_792_472,
    source_type: SourceType::Wikidata,
    name: "darcs patch format",
    extensions: &["darcspatch", "dpatch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
