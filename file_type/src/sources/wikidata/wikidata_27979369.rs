use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979369: FileFormat = FileFormat {
    id: 27_979_369,
    source_type: SourceType::Wikidata,
    name: "ReSample",
    extensions: &["srs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
