use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_75717796: FileFormat = FileFormat {
    id: 75_717_796,
    source_type: SourceType::Wikidata,
    name: "USRobotics firmware",
    extensions: &["usr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
