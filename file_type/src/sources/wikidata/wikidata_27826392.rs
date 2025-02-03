use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27826392: FileFormat = FileFormat {
    id: 27_826_392,
    source_type: SourceType::Wikidata,
    name: "Proxy Unrestricted Access Image",
    extensions: &["uai"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
