use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112821378: FileFormat = FileFormat {
    id: 112_821_378,
    source_type: SourceType::Wikidata,
    name: "Minolta 3D Scanner Camera File",
    extensions: &["cam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
