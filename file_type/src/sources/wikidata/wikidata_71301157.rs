use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_71301157: FileFormat = FileFormat {
    id: 71_301_157,
    puid: "wikidata/71301157",
    name: "WHIP! DWF Format",
    extensions: &["dwf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
