use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110995667: FileFormat = FileFormat {
    id: 110_995_667,
    source_type: SourceType::Wikidata,
    name: "VideoWave Scene",
    extensions: &["scn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
