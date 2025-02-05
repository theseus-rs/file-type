use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122904901: FileFormat = FileFormat {
    id: 122_904_901,
    source_type: SourceType::Wikidata,
    name: "PMTiles",
    extensions: &["pmtiles"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
