use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205626: FileFormat = FileFormat {
    id: 28_205_626,
    source_type: SourceType::Wikidata,
    name: "Sun icon",
    extensions: &["ico", "icon"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
