use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_75710254: FileFormat = FileFormat {
    id: 75_710_254,
    source_type: SourceType::Wikidata,
    name: "GeoGebra format, version 5",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};
