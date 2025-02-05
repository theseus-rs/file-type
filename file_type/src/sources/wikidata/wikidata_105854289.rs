use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854289: FileFormat = FileFormat {
    id: 105_854_289,
    source_type: SourceType::Wikidata,
    name: "ArcPad configuration",
    extensions: &["apx"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
