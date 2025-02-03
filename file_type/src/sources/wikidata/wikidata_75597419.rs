use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_75597419: FileFormat = FileFormat {
    id: 75_597_419,
    source_type: SourceType::Wikidata,
    name: "GeoGebra format, version 2.0",
    extensions: &["ggb"],
    media_types: &["application/vnd.geogebra.file"],
    internal_signatures: &[],
    related_formats: &[],
};
