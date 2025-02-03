use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117156378: FileFormat = FileFormat {
    id: 117_156_378,
    source_type: SourceType::Wikidata,
    name: "Pyro audio CD project",
    extensions: &["cwa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
