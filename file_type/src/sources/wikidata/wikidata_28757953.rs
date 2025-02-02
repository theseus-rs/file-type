use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757953: FileFormat = FileFormat {
    id: 28_757_953,
    source_type: SourceType::Wikidata,
    name: "HGT",
    extensions: &["hgt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
