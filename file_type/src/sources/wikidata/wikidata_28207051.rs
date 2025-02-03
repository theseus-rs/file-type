use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207051: FileFormat = FileFormat {
    id: 28_207_051,
    source_type: SourceType::Wikidata,
    name: "Pocket PC Bitmap",
    extensions: &["2bp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
