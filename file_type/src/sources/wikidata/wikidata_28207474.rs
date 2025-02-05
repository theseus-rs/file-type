use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207474: FileFormat = FileFormat {
    id: 28_207_474,
    source_type: SourceType::Wikidata,
    name: "Very Ordinary Rendering Toolkit file",
    extensions: &["pix"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
