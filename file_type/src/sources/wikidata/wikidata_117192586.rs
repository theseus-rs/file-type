use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117192586: FileFormat = FileFormat {
    id: 117_192_586,
    puid: "wikidata/117192586",
    name: "Generic Encapsulated PostScript Graphic Format",
    extensions: &["ai3", "ai4", "ai5", "ai6", "ai7", "ai8"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
