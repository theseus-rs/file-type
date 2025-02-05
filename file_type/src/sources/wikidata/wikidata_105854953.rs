use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854953: FileFormat = FileFormat {
    id: 105_854_953,
    source_type: SourceType::Wikidata,
    name: "ArcPad preferences",
    extensions: &["apx"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
