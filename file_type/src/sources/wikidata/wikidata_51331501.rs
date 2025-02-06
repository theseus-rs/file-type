use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51331501: FileFormat = FileFormat {
    id: 51_331_501,
    source_type: SourceType::Wikidata,
    name: "Hewlett Packard Vector Graphic Plotter File",
    extensions: &["plt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
