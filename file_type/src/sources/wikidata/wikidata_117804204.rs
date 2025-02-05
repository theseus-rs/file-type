use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117804204: FileFormat = FileFormat {
    id: 117_804_204,
    source_type: SourceType::Wikidata,
    name: "VideoImpression File",
    extensions: &["vif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
