use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113501336: FileFormat = FileFormat {
    id: 113_501_336,
    source_type: SourceType::Wikidata,
    name: "PageMaker Mac Document 6.5-7.0",
    extensions: &["p65", "pmd", "pmt", "t65"],
    media_types: &["application/vnd.pagemaker"],
    signatures: &[],
    related_formats: &[],
};
