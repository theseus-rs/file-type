use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856694: FileFormat = FileFormat {
    id: 105_856_694,
    source_type: SourceType::Wikidata,
    name: "Unified Scripture Format XML",
    extensions: &["usx"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
