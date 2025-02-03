use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205805: FileFormat = FileFormat {
    id: 28_205_805,
    source_type: SourceType::Wikidata,
    name: "Object File Format, binary variant",
    extensions: &["off"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
