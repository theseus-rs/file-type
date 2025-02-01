use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866205: FileFormat = FileFormat {
    id: 105_866_205,
    puid: "wikidata/105866205",
    name: "Image Packaging System Manifest (with rem)",
    extensions: &["p5m"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
