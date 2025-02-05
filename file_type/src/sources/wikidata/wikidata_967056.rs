use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_967056: FileFormat = FileFormat {
    id: 967_056,
    source_type: SourceType::Wikidata,
    name: "SoundFont",
    extensions: &["sf2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
