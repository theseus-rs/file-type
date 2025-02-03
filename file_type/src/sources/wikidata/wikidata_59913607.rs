use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59913607: FileFormat = FileFormat {
    id: 59_913_607,
    source_type: SourceType::Wikidata,
    name: "AV1 Image File Format",
    extensions: &["avif"],
    media_types: &["image/avif"],
    internal_signatures: &[],
    related_formats: &[],
};
