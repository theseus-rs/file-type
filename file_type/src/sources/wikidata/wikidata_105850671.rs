use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850671: FileFormat = FileFormat {
    id: 105_850_671,
    puid: "wikidata/105850671",
    name: "Google Earth placemark",
    extensions: &["kml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
