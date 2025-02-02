use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117536171: FileFormat = FileFormat {
    id: 117_536_171,
    source_type: SourceType::Wikidata,
    name: "3D Studio (DOS) 2D/3D Loft Object File",
    extensions: &["lft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
