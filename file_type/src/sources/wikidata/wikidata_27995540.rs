use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27995540: FileFormat = FileFormat {
    id: 27_995_540,
    source_type: SourceType::Wikidata,
    name: "Canon EOS C300 Custom Picture Profile",
    extensions: &["cpf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
