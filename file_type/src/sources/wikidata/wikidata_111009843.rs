use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009843: FileFormat = FileFormat {
    id: 111_009_843,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Sticker File format",
    extensions: &["sti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
