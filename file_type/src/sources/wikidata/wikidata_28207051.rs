use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207051: FileFormat = FileFormat {
    id: 28_207_051,
    source_type: SourceType::Wikidata,
    name: "Pocket PC Bitmap",
    extensions: &["2bp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
