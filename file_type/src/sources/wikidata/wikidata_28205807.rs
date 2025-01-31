use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205807: FileFormat = FileFormat {
    id: 28_205_807,
    puid: "wikidata/28205807",
    name: "Palette Format",
    extensions: &["pal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
