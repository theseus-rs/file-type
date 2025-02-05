use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28600470: FileFormat = FileFormat {
    id: 28_600_470,
    source_type: SourceType::Wikidata,
    name: "DER encoded RSA private key",
    extensions: &["key"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
