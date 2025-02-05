use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59390863: FileFormat = FileFormat {
    id: 59_390_863,
    source_type: SourceType::Wikidata,
    name: "Domino XML Database Export",
    extensions: &["dxl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
