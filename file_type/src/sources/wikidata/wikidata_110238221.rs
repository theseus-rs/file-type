use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110238221: FileFormat = FileFormat {
    id: 110_238_221,
    source_type: SourceType::Wikidata,
    name: "FrameImage",
    extensions: &["fmg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
