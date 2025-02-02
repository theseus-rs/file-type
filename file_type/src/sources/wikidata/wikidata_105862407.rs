use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862407: FileFormat = FileFormat {
    id: 105_862_407,
    source_type: SourceType::Wikidata,
    name: "Mac Compact Pro archive",
    extensions: &["cpt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
