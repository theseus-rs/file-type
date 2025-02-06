use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66689214: FileFormat = FileFormat {
    id: 66_689_214,
    source_type: SourceType::Wikidata,
    name: "Access Blank Database Template",
    extensions: &["mdn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
