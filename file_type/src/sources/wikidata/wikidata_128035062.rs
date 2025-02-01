use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128035062: FileFormat = FileFormat {
    id: 128_035_062,
    puid: "wikidata/128035062",
    name: "Protein Data Bank File 3.3",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
