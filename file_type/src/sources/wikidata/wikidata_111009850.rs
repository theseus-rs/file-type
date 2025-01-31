use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009850: FileFormat = FileFormat {
    id: 111_009_850,
    puid: "wikidata/111009850",
    name: "PrintMaster T-Shirt File format",
    extensions: &["tsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
