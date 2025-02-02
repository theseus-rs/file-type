use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205526: FileFormat = FileFormat {
    id: 28_205_526,
    source_type: SourceType::Wikidata,
    name: "Icon library",
    extensions: &["icl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
