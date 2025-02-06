use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66210170: FileFormat = FileFormat {
    id: 66_210_170,
    source_type: SourceType::Wikidata,
    name: "FrameMaker Book file format",
    extensions: &["book"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
