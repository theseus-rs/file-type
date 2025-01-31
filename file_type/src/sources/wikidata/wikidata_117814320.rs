use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117814320: FileFormat = FileFormat {
    id: 117_814_320,
    puid: "wikidata/117814320",
    name: "Working Model 3D Document",
    extensions: &["wm3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
