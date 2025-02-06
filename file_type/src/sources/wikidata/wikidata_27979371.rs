use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979371: FileFormat = FileFormat {
    id: 27_979_371,
    source_type: SourceType::Wikidata,
    name: "EBU Timed Text",
    extensions: &["ttml"],
    media_types: &["application/ttml+xml"],
    signatures: &[],
    related_formats: &[],
};
