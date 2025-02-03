use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28600470: FileFormat = FileFormat {
    id: 28_600_470,
    source_type: SourceType::Wikidata,
    name: "DER encoded RSA private key",
    extensions: &["key"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
