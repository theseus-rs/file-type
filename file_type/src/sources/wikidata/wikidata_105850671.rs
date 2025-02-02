use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850671: FileFormat = FileFormat {
    id: 105_850_671,
    source_type: SourceType::Wikidata,
    name: "Google Earth placemark",
    extensions: &["kml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
