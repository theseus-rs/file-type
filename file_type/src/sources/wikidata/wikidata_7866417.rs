use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_7866417: FileFormat = FileFormat {
    id: 7_866_417,
    source_type: SourceType::Wikidata,
    name: "USGS DEM",
    extensions: &["dem"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
