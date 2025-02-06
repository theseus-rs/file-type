use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_196765: FileFormat = FileFormat {
    id: 196_765,
    source_type: SourceType::Wikidata,
    name: "revocation list",
    extensions: &["crl"],
    media_types: &["application/pkix-crl"],
    signatures: &[],
    related_formats: &[],
};
