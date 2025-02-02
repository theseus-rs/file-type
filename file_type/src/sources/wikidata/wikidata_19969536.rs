use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_19969536: FileFormat = FileFormat {
    id: 19_969_536,
    source_type: SourceType::Wikidata,
    name: "DSV version 6 format",
    extensions: &["dsv", "dsv6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
