use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122074310: FileFormat = FileFormat {
    id: 122_074_310,
    puid: "wikidata/122074310",
    name: "SmartScore File",
    extensions: &["fin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
