use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115606990: FileFormat = FileFormat {
    id: 115_606_990,
    source_type: SourceType::Wikidata,
    name: "VCD Layout File",
    extensions: &["vcl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
