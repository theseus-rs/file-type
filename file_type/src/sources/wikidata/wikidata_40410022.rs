use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_40410022: FileFormat = FileFormat {
    id: 40_410_022,
    puid: "wikidata/40410022",
    name: "Feather",
    extensions: &["feather"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
