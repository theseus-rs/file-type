use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_11231091: FileFormat = FileFormat {
    id: 11_231_091,
    puid: "wikidata/11231091",
    name: "MASL",
    extensions: &["Msl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
