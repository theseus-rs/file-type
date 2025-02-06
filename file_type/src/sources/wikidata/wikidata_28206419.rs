use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206419: FileFormat = FileFormat {
    id: 28_206_419,
    source_type: SourceType::Wikidata,
    name: "LuraWave JPEG-2000 Code Stream Format",
    extensions: &["jpc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
