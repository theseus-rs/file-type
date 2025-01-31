use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263007: FileFormat = FileFormat {
    id: 111_263_007,
    puid: "wikidata/111263007",
    name: "Velvet Studio sample",
    extensions: &["ase"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
