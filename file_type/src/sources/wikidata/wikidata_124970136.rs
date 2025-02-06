use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124970136: FileFormat = FileFormat {
    id: 124_970_136,
    source_type: SourceType::Wikidata,
    name: "MIX status file",
    extensions: &["mixstatus"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
