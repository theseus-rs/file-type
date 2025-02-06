use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122260642: FileFormat = FileFormat {
    id: 122_260_642,
    source_type: SourceType::Wikidata,
    name: "JACOsub",
    extensions: &["jss"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
