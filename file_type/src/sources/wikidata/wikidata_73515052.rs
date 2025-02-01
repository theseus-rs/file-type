use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_73515052: FileFormat = FileFormat {
    id: 73_515_052,
    puid: "wikidata/73515052",
    name: "Microsoft Printer Definition",
    extensions: &["prd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
