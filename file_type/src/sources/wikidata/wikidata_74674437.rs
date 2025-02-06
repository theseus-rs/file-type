use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_74674437: FileFormat = FileFormat {
    id: 74_674_437,
    source_type: SourceType::Wikidata,
    name: "Kindle app book info",
    extensions: &["ticr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
