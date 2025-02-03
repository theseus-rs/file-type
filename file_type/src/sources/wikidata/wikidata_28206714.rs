use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206714: FileFormat = FileFormat {
    id: 28_206_714,
    source_type: SourceType::Wikidata,
    name: "Portable Anymap",
    extensions: &["pnm"],
    media_types: &["image/x-portable-anymap"],
    internal_signatures: &[],
    related_formats: &[],
};
