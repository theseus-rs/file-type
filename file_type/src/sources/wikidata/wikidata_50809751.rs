use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50809751: FileFormat = FileFormat {
    id: 50_809_751,
    puid: "wikidata/50809751",
    name: "Portable Database, version 1",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
