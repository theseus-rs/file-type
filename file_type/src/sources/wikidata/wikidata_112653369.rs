use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112653369: FileFormat = FileFormat {
    id: 112_653_369,
    source_type: SourceType::Wikidata,
    name: "Astound Draw backup file",
    extensions: &["ad~"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
