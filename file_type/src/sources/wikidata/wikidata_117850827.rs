use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117850827: FileFormat = FileFormat {
    id: 117_850_827,
    puid: "wikidata/117850827",
    name: "OAZ Fax file",
    extensions: &["oaz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
