use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34305583: FileFormat = FileFormat {
    id: 34_305_583,
    source_type: SourceType::Wikidata,
    name: "Racket script",
    extensions: &["rkt", "rktd", "rktl"],
    media_types: &["application/x-racket", "text/x-racket"],
    signatures: &[],
    related_formats: &[],
};
