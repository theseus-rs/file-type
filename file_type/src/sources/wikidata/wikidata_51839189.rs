use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51839189: FileFormat = FileFormat {
    id: 51_839_189,
    source_type: SourceType::Wikidata,
    name: "Broderbund Print Shop Deluxe Pamphlet",
    extensions: &["pdp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
