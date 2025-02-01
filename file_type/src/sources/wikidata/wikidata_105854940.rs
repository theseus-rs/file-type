use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854940: FileFormat = FileFormat {
    id: 105_854_940,
    puid: "wikidata/105854940",
    name: "Microsoft Assistance Markup Language",
    extensions: &["aml"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
