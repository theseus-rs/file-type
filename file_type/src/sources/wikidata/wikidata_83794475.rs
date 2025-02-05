use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83794475: FileFormat = FileFormat {
    id: 83_794_475,
    source_type: SourceType::Wikidata,
    name: "FO File",
    extensions: &["fo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
