use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118640510: FileFormat = FileFormat {
    id: 118_640_510,
    source_type: SourceType::Wikidata,
    name: "Anime Studio File",
    extensions: &["anme"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
