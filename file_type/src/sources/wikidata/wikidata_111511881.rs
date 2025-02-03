use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111511881: FileFormat = FileFormat {
    id: 111_511_881,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcInfo Coverage Annotation File",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
