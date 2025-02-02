use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111332609: FileFormat = FileFormat {
    id: 111_332_609,
    source_type: SourceType::Wikidata,
    name: "Orion Sampler program",
    extensions: &["osp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
