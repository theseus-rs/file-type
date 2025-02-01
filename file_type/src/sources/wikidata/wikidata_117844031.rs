use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117844031: FileFormat = FileFormat {
    id: 117_844_031,
    puid: "wikidata/117844031",
    name: "Inset Systems IGF format",
    extensions: &["igf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
