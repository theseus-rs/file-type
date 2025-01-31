use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009843: FileFormat = FileFormat {
    id: 111_009_843,
    puid: "wikidata/111009843",
    name: "PrintMaster Sticker File format",
    extensions: &["sti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
