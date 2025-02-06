use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47538955: FileFormat = FileFormat {
    id: 47_538_955,
    source_type: SourceType::Wikidata,
    name: "AutoLISP Menu Source File",
    extensions: &["mnl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
