use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34287064: FileFormat = FileFormat {
    id: 34_287_064,
    source_type: SourceType::Wikidata,
    name: "Professor Calhoon quiz file",
    extensions: &["pc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
