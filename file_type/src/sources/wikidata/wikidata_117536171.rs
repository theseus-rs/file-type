use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117536171: FileFormat = FileFormat {
    id: 117_536_171,
    puid: "wikidata/117536171",
    name: "3D Studio (DOS) 2D/3D Loft Object File",
    extensions: &["lft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
