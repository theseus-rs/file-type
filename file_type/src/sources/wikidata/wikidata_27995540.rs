use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27995540: FileFormat = FileFormat {
    id: 27_995_540,
    source_type: SourceType::Wikidata,
    name: "Canon EOS C300 Custom Picture Profile",
    extensions: &["cpf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
