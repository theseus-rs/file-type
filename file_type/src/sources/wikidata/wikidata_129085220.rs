use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129085220: FileFormat = FileFormat {
    id: 129_085_220,
    source_type: SourceType::Wikidata,
    name: "elpi file format",
    extensions: &["elpi"],
    media_types: &["text/x-elpi"],
    signatures: &[],
    related_formats: &[],
};
