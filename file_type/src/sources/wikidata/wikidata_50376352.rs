use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50376352: FileFormat = FileFormat {
    id: 50_376_352,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcGlobe Document",
    extensions: &["3dd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
