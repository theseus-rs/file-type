use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1760748: FileFormat = FileFormat {
    id: 1_760_748,
    source_type: SourceType::Wikidata,
    name: "Konqueror website archive",
    extensions: &["war"],
    media_types: &["application/x-webarchive"],
    internal_signatures: &[],
    related_formats: &[],
};
