use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2011664: FileFormat = FileFormat {
    id: 2_011_664,
    source_type: SourceType::Wikidata,
    name: "Object File Format",
    extensions: &["off"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
