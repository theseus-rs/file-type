use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7391833: FileFormat = FileFormat {
    id: 7_391_833,
    source_type: SourceType::Wikidata,
    name: "SND",
    extensions: &["snd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
