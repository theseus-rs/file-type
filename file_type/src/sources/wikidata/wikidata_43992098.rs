use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_43992098: FileFormat = FileFormat {
    id: 43_992_098,
    source_type: SourceType::Wikidata,
    name: "Extensible Metadata Platform Packet",
    extensions: &["xmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
