use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131303765: FileFormat = FileFormat {
    id: 131_303_765,
    puid: "wikidata/131303765",
    name: "TL-b source code file",
    extensions: &["tlb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
