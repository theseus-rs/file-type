use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117156378: FileFormat = FileFormat {
    id: 117_156_378,
    source_type: SourceType::Wikidata,
    name: "Pyro audio CD project",
    extensions: &["cwa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
