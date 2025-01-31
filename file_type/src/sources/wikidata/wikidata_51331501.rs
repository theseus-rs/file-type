use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51331501: FileFormat = FileFormat {
    id: 51_331_501,
    puid: "wikidata/51331501",
    name: "Hewlett Packard Vector Graphic Plotter File",
    extensions: &["plt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
