use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125134354: FileFormat = FileFormat {
    id: 125_134_354,
    source_type: SourceType::Wikidata,
    name: "YAM Folder Configuration",
    extensions: &["fconfig"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
