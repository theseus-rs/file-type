use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114075837: FileFormat = FileFormat {
    id: 114_075_837,
    source_type: SourceType::Wikidata,
    name: "Media Descriptor Sidecar File",
    extensions: &["mds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
