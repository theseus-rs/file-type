use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114075837: FileFormat = FileFormat {
    id: 114_075_837,
    source_type: SourceType::Wikidata,
    name: "Media Descriptor Sidecar File",
    extensions: &["mds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
