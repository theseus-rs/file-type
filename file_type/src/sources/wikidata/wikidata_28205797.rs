use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205797: FileFormat = FileFormat {
    id: 28_205_797,
    source_type: SourceType::Wikidata,
    name: "Canvas Picture Format",
    extensions: &["cnv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
