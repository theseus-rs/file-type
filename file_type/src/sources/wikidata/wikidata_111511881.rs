use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111511881: FileFormat = FileFormat {
    id: 111_511_881,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcInfo Coverage Annotation File",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
