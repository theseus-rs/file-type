use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87765717: FileFormat = FileFormat {
    id: 87_765_717,
    source_type: SourceType::Wikidata,
    name: "EIOffice Document",
    extensions: &["eio"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
