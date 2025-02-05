use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122904069: FileFormat = FileFormat {
    id: 122_904_069,
    source_type: SourceType::Wikidata,
    name: "MBTiles",
    extensions: &["mbtiles"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
