use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975884: FileFormat = FileFormat {
    id: 28_975_884,
    puid: "wikidata/28975884",
    name: "Geometry Data File",
    extensions: &["gdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
