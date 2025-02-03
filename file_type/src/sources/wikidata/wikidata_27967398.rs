use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967398: FileFormat = FileFormat {
    id: 27_967_398,
    source_type: SourceType::Wikidata,
    name: "AdLib Visual Composer / Roland Synthesizer song",
    extensions: &["rol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
