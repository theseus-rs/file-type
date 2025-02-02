use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206310: FileFormat = FileFormat {
    id: 28_206_310,
    source_type: SourceType::Wikidata,
    name: "Analyze HDR",
    extensions: &["hdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
