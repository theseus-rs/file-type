use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50498951: FileFormat = FileFormat {
    id: 50_498_951,
    source_type: SourceType::Wikidata,
    name: "OGR GFS File",
    extensions: &["gfs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
