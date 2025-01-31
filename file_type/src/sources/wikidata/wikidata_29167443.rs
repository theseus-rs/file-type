use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29167443: FileFormat = FileFormat {
    id: 29_167_443,
    puid: "wikidata/29167443",
    name: "OME-TIFF",
    extensions: &["ome.tiff"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
