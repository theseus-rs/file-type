use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50498951: FileFormat = FileFormat {
    id: 50_498_951,
    source_type: SourceType::Wikidata,
    name: "OGR GFS File",
    extensions: &["gfs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
