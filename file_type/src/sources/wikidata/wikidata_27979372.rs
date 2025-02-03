use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979372: FileFormat = FileFormat {
    id: 27_979_372,
    source_type: SourceType::Wikidata,
    name: "Kate",
    extensions: &["ogx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
