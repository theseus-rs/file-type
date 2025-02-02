use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125134588: FileFormat = FileFormat {
    id: 125_134_588,
    source_type: SourceType::Wikidata,
    name: "YAM Unique ID Listing",
    extensions: &["uidl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
