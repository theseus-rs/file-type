use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119786070: FileFormat = FileFormat {
    id: 119_786_070,
    source_type: SourceType::Wikidata,
    name: "MasterCook Tips File",
    extensions: &["mtf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
