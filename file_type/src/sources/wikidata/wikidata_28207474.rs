use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207474: FileFormat = FileFormat {
    id: 28_207_474,
    source_type: SourceType::Wikidata,
    name: "Very Ordinary Rendering Toolkit file",
    extensions: &["pix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
