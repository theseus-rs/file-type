use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009843: FileFormat = FileFormat {
    id: 111_009_843,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Sticker File format",
    extensions: &["sti"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
