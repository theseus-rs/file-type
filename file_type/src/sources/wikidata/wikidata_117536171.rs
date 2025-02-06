use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117536171: FileFormat = FileFormat {
    id: 117_536_171,
    source_type: SourceType::Wikidata,
    name: "3D Studio (DOS) 2D/3D Loft Object File",
    extensions: &["lft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
