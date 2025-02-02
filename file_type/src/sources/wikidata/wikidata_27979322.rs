use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979322: FileFormat = FileFormat {
    id: 27_979_322,
    source_type: SourceType::Wikidata,
    name: "Photoshop Curve",
    extensions: &["crv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
