use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_17029350: FileFormat = FileFormat {
    id: 17_029_350,
    puid: "wikidata/17029350",
    name: "Image Cytometry Standard",
    extensions: &["ics", "ics", "ids", "ids"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
