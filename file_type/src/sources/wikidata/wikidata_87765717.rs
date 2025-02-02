use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87765717: FileFormat = FileFormat {
    id: 87_765_717,
    source_type: SourceType::Wikidata,
    name: "EIOffice Document",
    extensions: &["eio"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
