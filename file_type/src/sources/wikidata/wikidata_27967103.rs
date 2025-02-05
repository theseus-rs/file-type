use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967103: FileFormat = FileFormat {
    id: 27_967_103,
    source_type: SourceType::Wikidata,
    name: "Nintendo GameCube/Wii BRSTM",
    extensions: &["brstm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
