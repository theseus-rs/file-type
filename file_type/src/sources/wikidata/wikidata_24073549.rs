use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_24073549: FileFormat = FileFormat {
    id: 24_073_549,
    source_type: SourceType::Wikidata,
    name: "SFZ",
    extensions: &["sfz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
