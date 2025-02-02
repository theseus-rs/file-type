use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206332: FileFormat = FileFormat {
    id: 28_206_332,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Green Component",
    extensions: &["g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
