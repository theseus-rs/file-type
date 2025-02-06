use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127604990: FileFormat = FileFormat {
    id: 127_604_990,
    source_type: SourceType::Wikidata,
    name: "Awk script",
    extensions: &["awk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
