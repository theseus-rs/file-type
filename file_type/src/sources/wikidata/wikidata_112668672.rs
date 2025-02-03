use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112668672: FileFormat = FileFormat {
    id: 112_668_672,
    source_type: SourceType::Wikidata,
    name: "Lightscape Blocks",
    extensions: &["blk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
