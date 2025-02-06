use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127378243: FileFormat = FileFormat {
    id: 127_378_243,
    source_type: SourceType::Wikidata,
    name: "FreeBASIC Header File",
    extensions: &["bi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
