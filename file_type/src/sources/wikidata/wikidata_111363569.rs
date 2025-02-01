use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111363569: FileFormat = FileFormat {
    id: 111_363_569,
    puid: "wikidata/111363569",
    name: "id Software Music Format (700Hz)",
    extensions: &["wlf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
