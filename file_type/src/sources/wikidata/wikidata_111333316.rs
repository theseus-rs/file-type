use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111333316: FileFormat = FileFormat {
    id: 111_333_316,
    puid: "wikidata/111333316",
    name: "WAVmaker program file",
    extensions: &["prg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
