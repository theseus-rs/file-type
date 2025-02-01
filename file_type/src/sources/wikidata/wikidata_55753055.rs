use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55753055: FileFormat = FileFormat {
    id: 55_753_055,
    puid: "wikidata/55753055",
    name: "Redcode Metadata File",
    extensions: &["rmd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
