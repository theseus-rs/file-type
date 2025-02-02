use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_737207: FileFormat = FileFormat {
    id: 737_207,
    source_type: SourceType::Wikidata,
    name: "RealVideo",
    extensions: &["rv"],
    media_types: &["application/vnd.rn-realmedia"],
    internal_signatures: &[],
    related_formats: &[],
};
