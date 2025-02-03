use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_115606990: FileFormat = FileFormat {
    id: 115_606_990,
    source_type: SourceType::Wikidata,
    name: "VCD Layout File",
    extensions: &["vcl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
