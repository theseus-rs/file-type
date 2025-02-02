use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51789800: FileFormat = FileFormat {
    id: 51_789_800,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visio Drawing, version 5",
    extensions: &["vsd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
