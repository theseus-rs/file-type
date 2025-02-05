use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131413711: FileFormat = FileFormat {
    id: 131_413_711,
    source_type: SourceType::Wikidata,
    name: "VisualProlog grammar file format",
    extensions: &["vipgrm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
