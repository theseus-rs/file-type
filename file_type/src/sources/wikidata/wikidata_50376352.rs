use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50376352: FileFormat = FileFormat {
    id: 50_376_352,
    source_type: SourceType::Wikidata,
    name: "ESRI ArcGlobe Document",
    extensions: &["3dd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
