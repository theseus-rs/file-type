use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122904069: FileFormat = FileFormat {
    id: 122_904_069,
    source_type: SourceType::Wikidata,
    name: "MBTiles",
    extensions: &["mbtiles"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
