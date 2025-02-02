use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_627554: FileFormat = FileFormat {
    id: 627_554,
    source_type: SourceType::Wikidata,
    name: "certificate signing request",
    extensions: &["csr", "p10"],
    media_types: &["application/pkcs10"],
    internal_signatures: &[],
    related_formats: &[],
};
