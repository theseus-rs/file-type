use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112943858: FileFormat = FileFormat {
    id: 112_943_858,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 material definition file",
    extensions: &["gmf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
