use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_92440742: FileFormat = FileFormat {
    id: 92_440_742,
    source_type: SourceType::Wikidata,
    name: "Spider 2D image",
    extensions: &["spider"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
