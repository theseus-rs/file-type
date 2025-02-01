use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111009602: FileFormat = FileFormat {
    id: 111_009_602,
    puid: "wikidata/111009602",
    name: "PrintMaster Business Card File format",
    extensions: &["biz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
