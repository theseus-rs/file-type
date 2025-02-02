use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47512572: FileFormat = FileFormat {
    id: 47_512_572,
    source_type: SourceType::Wikidata,
    name: "man page",
    extensions: &["1", "2", "3", "4", "5", "6", "7", "8", "man"],
    media_types: &["application/x-troff-man", "text/troff"],
    internal_signatures: &[],
    related_formats: &[],
};
