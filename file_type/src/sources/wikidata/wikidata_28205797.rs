use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205797: FileFormat = FileFormat {
    id: 28_205_797,
    source_type: SourceType::Wikidata,
    name: "Canvas Picture Format",
    extensions: &["cnv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
