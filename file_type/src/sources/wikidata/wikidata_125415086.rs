use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125415086: FileFormat = FileFormat {
    id: 125_415_086,
    puid: "wikidata/125415086",
    name: "TCM document",
    extensions: &["tcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
