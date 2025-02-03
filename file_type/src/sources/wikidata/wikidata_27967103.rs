use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967103: FileFormat = FileFormat {
    id: 27_967_103,
    source_type: SourceType::Wikidata,
    name: "Nintendo GameCube/Wii BRSTM",
    extensions: &["brstm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
