use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205763: FileFormat = FileFormat {
    id: 28_205_763,
    source_type: SourceType::Wikidata,
    name: "Binary Information File",
    extensions: &["bif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
