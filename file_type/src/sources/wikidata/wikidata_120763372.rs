use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120763372: FileFormat = FileFormat {
    id: 120_763_372,
    source_type: SourceType::Wikidata,
    name: "Topo USA 3.0 File",
    extensions: &["tp3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
