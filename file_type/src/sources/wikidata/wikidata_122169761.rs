use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122169761: FileFormat = FileFormat {
    id: 122_169_761,
    source_type: SourceType::Wikidata,
    name: "Domain Cached Credentials",
    extensions: &["dcc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
