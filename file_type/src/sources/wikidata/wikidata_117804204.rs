use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117804204: FileFormat = FileFormat {
    id: 117_804_204,
    source_type: SourceType::Wikidata,
    name: "VideoImpression File",
    extensions: &["vif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
