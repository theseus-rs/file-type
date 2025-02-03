use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110238221: FileFormat = FileFormat {
    id: 110_238_221,
    source_type: SourceType::Wikidata,
    name: "FrameImage",
    extensions: &["fmg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
