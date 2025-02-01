use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_123194261: FileFormat = FileFormat {
    id: 123_194_261,
    puid: "wikidata/123194261",
    name: "Comodo Backup File",
    extensions: &["cbu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
