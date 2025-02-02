use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119999757: FileFormat = FileFormat {
    id: 119_999_757,
    source_type: SourceType::Wikidata,
    name: "DJ RingTone File",
    extensions: &["djr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
