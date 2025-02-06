use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51837533: FileFormat = FileFormat {
    id: 51_837_533,
    source_type: SourceType::Wikidata,
    name: "Visual FoxPro Database Container File",
    extensions: &["dcx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
