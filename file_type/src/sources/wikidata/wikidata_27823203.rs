use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823203: FileFormat = FileFormat {
    id: 27_823_203,
    source_type: SourceType::Wikidata,
    name: "Synalysis grammar file",
    extensions: &["grammar"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
