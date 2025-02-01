use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112668672: FileFormat = FileFormat {
    id: 112_668_672,
    puid: "wikidata/112668672",
    name: "Lightscape Blocks",
    extensions: &["blk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
