use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_47538955: FileFormat = FileFormat {
    id: 47_538_955,
    source_type: SourceType::Wikidata,
    name: "AutoLISP Menu Source File",
    extensions: &["mnl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
