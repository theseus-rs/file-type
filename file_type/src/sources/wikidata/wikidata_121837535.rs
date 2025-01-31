use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121837535: FileFormat = FileFormat {
    id: 121_837_535,
    puid: "wikidata/121837535",
    name: "OPML File 1.x",
    extensions: &["opml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
