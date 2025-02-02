use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116969050: FileFormat = FileFormat {
    id: 116_969_050,
    source_type: SourceType::Wikidata,
    name: "RS Red Storm File",
    extensions: &["rsb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
