use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126960675: FileFormat = FileFormat {
    id: 126_960_675,
    source_type: SourceType::Wikidata,
    name: "VAPI file",
    extensions: &["vapi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
