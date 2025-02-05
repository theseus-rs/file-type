use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125134588: FileFormat = FileFormat {
    id: 125_134_588,
    source_type: SourceType::Wikidata,
    name: "YAM Unique ID Listing",
    extensions: &["uidl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
