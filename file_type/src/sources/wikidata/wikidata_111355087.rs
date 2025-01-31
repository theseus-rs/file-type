use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355087: FileFormat = FileFormat {
    id: 111_355_087,
    puid: "wikidata/111355087",
    name: "G.711 mu-law US telephony format",
    extensions: &["ulw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
