use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114891689: FileFormat = FileFormat {
    id: 114_891_689,
    puid: "wikidata/114891689",
    name: "Quicken Rental Property Manager File",
    extensions: &["qrp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
