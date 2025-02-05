use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100235620: FileFormat = FileFormat {
    id: 100_235_620,
    source_type: SourceType::Wikidata,
    name: "FARO WorkSpace File",
    extensions: &["fws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
