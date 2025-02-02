use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206593: FileFormat = FileFormat {
    id: 28_206_593,
    source_type: SourceType::Wikidata,
    name: "MSX Interchange Format",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
