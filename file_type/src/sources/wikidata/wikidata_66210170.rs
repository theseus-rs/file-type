use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66210170: FileFormat = FileFormat {
    id: 66_210_170,
    source_type: SourceType::Wikidata,
    name: "FrameMaker Book file format",
    extensions: &["book"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
