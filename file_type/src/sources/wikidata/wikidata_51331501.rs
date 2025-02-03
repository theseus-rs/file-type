use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51331501: FileFormat = FileFormat {
    id: 51_331_501,
    source_type: SourceType::Wikidata,
    name: "Hewlett Packard Vector Graphic Plotter File",
    extensions: &["plt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
