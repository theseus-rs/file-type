use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206336: FileFormat = FileFormat {
    id: 28_206_336,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Blue Component",
    extensions: &["b"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
