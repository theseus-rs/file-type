use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_75717796: FileFormat = FileFormat {
    id: 75_717_796,
    source_type: SourceType::Wikidata,
    name: "USRobotics firmware",
    extensions: &["usr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
