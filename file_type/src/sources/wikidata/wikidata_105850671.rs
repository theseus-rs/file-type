use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850671: FileFormat = FileFormat {
    id: 105_850_671,
    source_type: SourceType::Wikidata,
    name: "Google Earth placemark",
    extensions: &["kml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
