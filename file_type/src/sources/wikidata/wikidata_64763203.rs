use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_64763203: FileFormat = FileFormat {
    id: 64_763_203,
    source_type: SourceType::Wikidata,
    name: "MapPoint template file format",
    extensions: &["ptt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
