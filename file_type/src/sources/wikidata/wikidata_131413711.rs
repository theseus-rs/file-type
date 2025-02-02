use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131413711: FileFormat = FileFormat {
    id: 131_413_711,
    source_type: SourceType::Wikidata,
    name: "VisualProlog grammar file format",
    extensions: &["vipgrm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
