use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854753: FileFormat = FileFormat {
    id: 105_854_753,
    source_type: SourceType::Wikidata,
    name: "ArcPad Graphics layer",
    extensions: &["apg"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
