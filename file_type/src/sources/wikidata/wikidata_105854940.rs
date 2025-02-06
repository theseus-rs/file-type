use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854940: FileFormat = FileFormat {
    id: 105_854_940,
    source_type: SourceType::Wikidata,
    name: "Microsoft Assistance Markup Language",
    extensions: &["aml"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
