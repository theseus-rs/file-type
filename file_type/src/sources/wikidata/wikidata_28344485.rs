use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28344485: FileFormat = FileFormat {
    id: 28_344_485,
    source_type: SourceType::Wikidata,
    name: "HKI",
    extensions: &["hki"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
