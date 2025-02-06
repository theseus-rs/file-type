use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111333316: FileFormat = FileFormat {
    id: 111_333_316,
    source_type: SourceType::Wikidata,
    name: "WAVmaker program file",
    extensions: &["prg"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
