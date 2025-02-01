use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50376352: FileFormat = FileFormat {
    id: 50_376_352,
    puid: "wikidata/50376352",
    name: "ESRI ArcGlobe Document",
    extensions: &["3dd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
