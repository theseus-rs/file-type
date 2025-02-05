use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111284149: FileFormat = FileFormat {
    id: 111_284_149,
    source_type: SourceType::Wikidata,
    name: "Bells, Whistles, Sound Boards module",
    extensions: &["gdm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
