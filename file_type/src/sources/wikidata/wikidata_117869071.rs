use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117869071: FileFormat = FileFormat {
    id: 117_869_071,
    source_type: SourceType::Wikidata,
    name: "Relisys file",
    extensions: &["tef"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
