use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_379770: FileFormat = FileFormat {
    id: 379_770,
    source_type: SourceType::Wikidata,
    name: "AVCHD",
    extensions: &["avchd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
