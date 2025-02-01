use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854810: FileFormat = FileFormat {
    id: 105_854_810,
    puid: "wikidata/105854810",
    name: "Windows Policy Administrative Template (with rem)",
    extensions: &["adm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
