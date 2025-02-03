use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122904901: FileFormat = FileFormat {
    id: 122_904_901,
    source_type: SourceType::Wikidata,
    name: "PMTiles",
    extensions: &["pmtiles"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
