use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27995540: FileFormat = FileFormat {
    id: 27_995_540,
    puid: "wikidata/27995540",
    name: "Canon EOS C300 Custom Picture Profile",
    extensions: &["cpf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
