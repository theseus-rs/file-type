use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3477565: FileFormat = FileFormat {
    id: 3_477_565,
    source_type: SourceType::Wikidata,
    name: "Secure Digital Container",
    extensions: &["sdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
