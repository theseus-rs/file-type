use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661259: FileFormat = FileFormat {
    id: 112_661_259,
    source_type: SourceType::Wikidata,
    name: "LightWave LScript File",
    extensions: &["ls"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
