use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206332: FileFormat = FileFormat {
    id: 28_206_332,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Green Component",
    extensions: &["g"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
