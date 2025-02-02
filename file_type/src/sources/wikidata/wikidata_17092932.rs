use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_17092932: FileFormat = FileFormat {
    id: 17_092_932,
    source_type: SourceType::Wikidata,
    name: "JPEG-XT",
    extensions: &["jpeg", "jpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
