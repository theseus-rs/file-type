use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7251429: FileFormat = FileFormat {
    id: 7_251_429,
    puid: "wikidata/7251429",
    name: "Protein Data Bank",
    extensions: &["pdb"],
    media_types: &["chemical/x-pdb"],
    internal_signatures: &[],
    related_formats: &[],
};
