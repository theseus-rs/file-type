use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43992098: FileFormat = FileFormat {
    id: 43_992_098,
    source_type: SourceType::Wikidata,
    name: "Extensible Metadata Platform Packet",
    extensions: &["xmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
