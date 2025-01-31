use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50809785: FileFormat = FileFormat {
    id: 50_809_785,
    puid: "wikidata/50809785",
    name: "Portable Database, version 3",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
