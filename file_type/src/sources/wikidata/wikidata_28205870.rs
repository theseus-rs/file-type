use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205870: FileFormat = FileFormat {
    id: 28_205_870,
    source_type: SourceType::Wikidata,
    name: "Crack Art",
    extensions: &["ca1", "ca2", "ca3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
