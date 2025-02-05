use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116957974: FileFormat = FileFormat {
    id: 116_957_974,
    source_type: SourceType::Wikidata,
    name: "ICN AT&T/Multigen",
    extensions: &["icn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
