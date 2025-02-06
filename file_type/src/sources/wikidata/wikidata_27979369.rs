use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979369: FileFormat = FileFormat {
    id: 27_979_369,
    source_type: SourceType::Wikidata,
    name: "ReSample",
    extensions: &["srs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
