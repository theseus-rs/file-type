use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207561: FileFormat = FileFormat {
    id: 28_207_561,
    source_type: SourceType::Wikidata,
    name: "Xim",
    extensions: &["xim"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
