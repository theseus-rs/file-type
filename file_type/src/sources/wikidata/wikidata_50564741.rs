use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50564741: FileFormat = FileFormat {
    id: 50_564_741,
    source_type: SourceType::Wikidata,
    name: "AVCHD Clip Information File",
    extensions: &["clpi", "cpi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
