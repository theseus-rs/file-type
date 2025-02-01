use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114075837: FileFormat = FileFormat {
    id: 114_075_837,
    puid: "wikidata/114075837",
    name: "Media Descriptor Sidecar File",
    extensions: &["mds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
