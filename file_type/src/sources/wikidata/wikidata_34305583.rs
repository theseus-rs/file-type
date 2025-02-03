use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34305583: FileFormat = FileFormat {
    id: 34_305_583,
    source_type: SourceType::Wikidata,
    name: "Racket script",
    extensions: &["rkt", "rktd", "rktl"],
    media_types: &["application/x-racket", "text/x-racket"],
    internal_signatures: &[],
    related_formats: &[],
};
