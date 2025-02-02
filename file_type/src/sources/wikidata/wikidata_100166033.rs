use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100166033: FileFormat = FileFormat {
    id: 100_166_033,
    source_type: SourceType::Wikidata,
    name: "Drawing Interchange Format (Binary) (Generic)",
    extensions: &["dxf"],
    media_types: &["image/vnd.dxf"],
    internal_signatures: &[],
    related_formats: &[],
};
