use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_104889134: FileFormat = FileFormat {
    id: 104_889_134,
    puid: "wikidata/104889134",
    name: "Propellerhead Reason Project File",
    extensions: &["reason", "rns"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
