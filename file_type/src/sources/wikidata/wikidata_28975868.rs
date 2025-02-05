use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975868: FileFormat = FileFormat {
    id: 28_975_868,
    source_type: SourceType::Wikidata,
    name: "OOGL SPHERE file",
    extensions: &["sph"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
