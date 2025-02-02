use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34287064: FileFormat = FileFormat {
    id: 34_287_064,
    source_type: SourceType::Wikidata,
    name: "Professor Calhoon quiz file",
    extensions: &["pc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
