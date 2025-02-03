use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116957974: FileFormat = FileFormat {
    id: 116_957_974,
    source_type: SourceType::Wikidata,
    name: "ICN AT&T/Multigen",
    extensions: &["icn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
