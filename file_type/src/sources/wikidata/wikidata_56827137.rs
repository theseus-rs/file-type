use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_56827137: FileFormat = FileFormat {
    id: 56_827_137,
    puid: "wikidata/56827137",
    name: "Nintendo DS cartridge file",
    extensions: &["nds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
