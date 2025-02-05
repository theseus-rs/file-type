use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116969050: FileFormat = FileFormat {
    id: 116_969_050,
    source_type: SourceType::Wikidata,
    name: "RS Red Storm File",
    extensions: &["rsb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
