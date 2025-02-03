use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131417573: FileFormat = FileFormat {
    id: 131_417_573,
    source_type: SourceType::Wikidata,
    name: "FRACT file",
    extensions: &["FRACT"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
