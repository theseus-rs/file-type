use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127378243: FileFormat = FileFormat {
    id: 127_378_243,
    source_type: SourceType::Wikidata,
    name: "FreeBASIC Header File",
    extensions: &["bi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
