use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_64763203: FileFormat = FileFormat {
    id: 64_763_203,
    source_type: SourceType::Wikidata,
    name: "MapPoint template file format",
    extensions: &["ptt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
