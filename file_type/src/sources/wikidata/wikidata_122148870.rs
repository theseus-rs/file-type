use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122148870: FileFormat = FileFormat {
    id: 122_148_870,
    puid: "wikidata/122148870",
    name: "My Logo Maker File",
    extensions: &["myf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
