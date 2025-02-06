use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967398: FileFormat = FileFormat {
    id: 27_967_398,
    source_type: SourceType::Wikidata,
    name: "AdLib Visual Composer / Roland Synthesizer song",
    extensions: &["rol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
